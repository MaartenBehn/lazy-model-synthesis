use std::ffi::c_float;
use crate::util::state_saver::TickType;
use std::time::Duration;
use fastrand::Rng;
use num_enum::TryFromPrimitive;
use octa_force::gui::Gui;
use octa_force::anyhow::*;
use octa_force::{BaseApp, egui, glam};
use octa_force::egui::{Align, FontId, Frame, Id, Layout, Pos2, RichText, Ui, Widget};
use octa_force::egui::FontFamily::Proportional;
use octa_force::egui::panel::Side;
use octa_force::egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::{IVec2, ivec2, vec2, Vec2};
use octa_force::puffin_egui::puffin;
use octa_force::vulkan::ash::vk::AttachmentLoadOp;
use crate::depth_search::node::DepthNode;
use crate::depth_search::node_manager::DepthNodeManager;
use crate::depth_search::value::DepthValue;
use crate::dispatcher::{DepthTreeDispatcherT, TREE_APPLY_INDEX, TREE_BUILD_INDEX, TREE_CHECK_INDEX};
use crate::grid::grid::{Grid, ValueData};
use crate::grid::identifier::{ChunkNodeIndex, GlobalPos, PackedChunkNodeIndex};
use crate::dispatcher::vec_dispatcher::VecTreeDispatcher;
use crate::grid::render::node_render_data::NUM_VALUE_TYPES;
use crate::grid::render::renderer::GridRenderer;
use crate::grid::render::selector::Selector;
use crate::grid::rules::{get_example_rules, NUM_REQS, NUM_VALUES, ValueType};
use crate::LazyModelSynthesis;
use crate::general_data_structure::identifier::IdentifierConverterT;
use crate::general_data_structure::node_storage::NodeStorageT;
use crate::general_data_structure::value::{ValueDataT};
use crate::util::state_saver::StateSaver;

const CHUNK_SIZE: usize = 32;
const DEBUG_MODE: bool = true;

pub struct GridDebugDepthVisulation {
    pub gui: Gui,
    
    pub state_saver: StateSaver<
        DepthNodeManager<
            Grid<DepthNode<ValueData, ChunkNodeIndex>, DepthValue<ValueData>>,
            VecTreeDispatcher,
            GlobalPos, 
            ChunkNodeIndex, 
            PackedChunkNodeIndex,
            ValueData,
            true,
        >
    >,

    pub grid_renderer: GridRenderer,
    pub selector: Selector,

    run: bool,
    run_ticks_per_frame: usize,
    pointer_pos_in_grid: Option<Vec2>,
}

impl GridDebugDepthVisulation {
    pub fn new(base: &mut BaseApp<LazyModelSynthesis>) -> Result<Self> {

        let mut grid = Grid::new(CHUNK_SIZE, Some(ValueData::new(ValueType::Stone)));
        grid.add_chunk(IVec2::ZERO);
        grid.rules = get_example_rules();

        let node_manager = DepthNodeManager::new(grid, NUM_VALUES, NUM_REQS);
        
        let state_saver = StateSaver::from_state(node_manager, 100);

        let mut gui = Gui::new(
            &base.context,
            base.swapchain.format,
            base.swapchain.depth_format,
            &base.window,
            base.num_frames
        )?;

        let grid_renderer = GridRenderer::new(&mut base.context, &mut gui.renderer, base.num_frames, CHUNK_SIZE, 1)?;
        let selector = Selector::new();

        Ok(GridDebugDepthVisulation {
            state_saver,
            gui,
            grid_renderer,
            selector,
            run: false,
            run_ticks_per_frame: 10,
            pointer_pos_in_grid: None,
        })
    }

    pub fn update(
        &mut self,
        base: &mut BaseApp<LazyModelSynthesis>,
        frame_index: usize,
        _delta_time: Duration,
    ) -> Result<()> {
        
        if base.controls.mouse_left && self.selector.last_selected.is_some() && self.selector.value_type_to_place.is_some() {
            self.state_saver.get_state_mut().select_value(
                GlobalPos(self.selector.last_selected.unwrap()), 
                ValueData::new(self.selector.value_type_to_place.unwrap())
            );
        }
        
        if self.run  {
            self.state_saver.set_next_tick(TickType::ForwardSave);
            for _ in 0..self.run_ticks_per_frame {
                self.state_saver.tick();
            }
        } else {
            self.state_saver.tick();
        }
        self.state_saver.set_next_tick(TickType::None);
        
        if DEBUG_MODE {
            let mut tree_d = self.state_saver.get_state().tree_dispatcher.clone();

            // Place one automatic at the start
            if tree_d.pop_tree_build_tick().is_none() &&  tree_d.pop_tree_apply_tick().is_none() {
                let gi = GlobalPos(ivec2(fastrand::i32(0..32), fastrand::i32(0..32)));
                let fi = self.state_saver.get_state().node_storage.fast_from_general(gi);
                let node = self.state_saver.get_state().node_storage.get_node(fi);
                let v = &node.value;
                let next_vt = if v.is_some() {
                    let vt = ValueType::try_from_primitive(v.unwrap().value_data.get_value_nr()).unwrap();
                    if vt == ValueType::Stone {
                        ValueType::Sand
                    } else if vt == ValueType::Grass {
                        ValueType::Stone
                    } else {
                        ValueType::Stone
                    }
                } else {
                    ValueType::Grass
                };

                self.state_saver.get_state_mut().select_value(gi, ValueData::new(next_vt));

                self.run = false;
            }
        }
        
        self.selector.add_to_render_data(self.pointer_pos_in_grid, &mut self.state_saver.get_state_mut().node_storage);

        self.grid_renderer.set_chunk_data(0, CHUNK_SIZE, &self.state_saver.get_state().node_storage.chunks[0].render_data);
        self.grid_renderer.update(&mut base.context, base.swapchain.format, frame_index);

        self.selector.clear_from_render_data(&mut self.state_saver.get_state_mut().node_storage);
        
        Ok(())
    }

    pub fn record_render_commands(
        &mut self,
        base: &mut BaseApp<LazyModelSynthesis>,
        frame_index: usize,
    ) -> Result<()> {

        let command_buffer = &base.command_buffers[frame_index];

        let size = base.swapchain.size;
        let swap_chain_image = &base.swapchain.images_and_views[frame_index].image;
        let swap_chain_view = &base.swapchain.images_and_views[frame_index].view;
        let swap_chain_depth_view = &base.swapchain.depht_images_and_views[frame_index].view;

        self.grid_renderer.render(command_buffer, frame_index);

        command_buffer.begin_rendering(swap_chain_view, swap_chain_depth_view, size, AttachmentLoadOp::CLEAR, None);

        self.gui.cmd_draw(command_buffer, size, frame_index, &base.window, &base.context, |ctx| {
            // Get current context style
            let mut style = (*ctx.style()).clone();

            // Redefine text_styles
            style.text_styles = [
                (Heading, FontId::new(20.0, Proportional)),
                (Body, FontId::new(15.0, Proportional)),
                (Monospace, FontId::new(14.0, Proportional)),
                (Button, FontId::new(15.0, Proportional)),
                (Small, FontId::new(10.0, Proportional)),
            ].into();

            // Mutate global style with above changes
            ctx.set_style(style);
            
            
            egui::SidePanel::new(Side::Left, Id::new("Side Panel")).show(ctx, |ui| {
                puffin::profile_scope!("Left Panel");

                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    div(ui, |ui| {
                        ui.heading("Depth Tree Grid (Debug Mode)");
                    });


                    div(ui, |ui| {
                        ui.label("Tick: ");

                        if ui.button("<<<").clicked() {
                            self.run = false;
                            self.state_saver.set_next_tick(TickType::Back);
                        }

                        if ui.button(">>>").clicked() {
                            self.run = false;
                            self.state_saver.set_next_tick(TickType::ForwardSave);
                        }
                    });

                    div(ui, |ui| {
                        ui.label("saved ticks:");
                    });

                    div(ui, |ui| {
                        let (current_saved, num_saved) = self.state_saver.get_step_state();
                        egui::ProgressBar::new(1.0 - (current_saved as f32 / num_saved as f32)).ui(ui);
                    });

                    div(ui, |ui| {
                        ui.checkbox(&mut self.run, "run");
                        
                        ui.label("Ticks per frame: ");

                        ui.add(
                            egui::DragValue::new(&mut self.run_ticks_per_frame)
                                .speed(1)
                                .range(1..=100),
                        );
                        
                    });

                    
                    div(ui, |ui| {
                        if ui.button("clear").clicked() {
                            self.run = false;
                            self.state_saver.reset()
                        }
                    });

                    ui.separator();
                    
                    
                    div(ui, |ui| {
                        ui.label("Place: ");

                        for i in 0..NUM_VALUE_TYPES {
                            let value_type = ValueType::try_from_primitive(i).unwrap();
                            
                            let mut checked = self.selector.value_type_to_place == Some(value_type); 
                            ui.checkbox(&mut checked, format!("{:?}", value_type));
                            if checked {
                                self.selector.value_type_to_place = Some(value_type);
                            }
                        }
                        
                    });

                    ui.separator();

                    div(ui, |ui| {
                        ui.heading("Selected Node");
                    });

                    if let Some(pos) = self.selector.last_selected {
                        div(ui, |ui| {
                            ui.label(format!("Pos: [{:0>2} {:0>2}]", pos.x, pos.y));
                        });

                        let chunk_node_index = self.state_saver.get_state_mut().node_storage
                            .get_chunk_and_node_index_from_global_pos(GlobalPos(pos));
                        
                        let data = self.state_saver.get_state().node_storage
                            .chunks[chunk_node_index.chunk_index]
                            .render_data[chunk_node_index.node_index];

                        let node = self.state_saver
                            .get_state()
                            .node_storage
                            .get_node(chunk_node_index);

                        for (i, (value_data, tree_index)) in node.tree_nodes.iter().enumerate() {
                            let add_queue = data.get_queue(*value_data, TREE_CHECK_INDEX);
                            let remove_queue = data.get_queue(*value_data, TREE_BUILD_INDEX);
                            let select_queue = data.get_queue(*value_data, TREE_APPLY_INDEX);
                            
                            div(ui, |ui| {
                                ui.label(
                                    format!(
                                        "{}Tree Node: {} -> {:?} ({} {} {})",
                                        if node.chosen_node_index == Some(i) {"> "} else {""},
                                        tree_index,
                                        value_data.value_type,
                                        if add_queue {"Check"} else {""},
                                        if remove_queue {"Build"} else {""},
                                        if select_queue {"Apply"} else {""},
                                        
                                    ));
                            });
                        }
                        
                        if node.fixed_value.is_some() {
                            ui.label(format!("Fixed Value: {:?}",  node.fixed_value.unwrap().value_type));
                        }

                        ui.label("Tree Reqs at:");
                        for (fast, reqs_at) in node.reqs_at.iter() {
                            let identifier = self.state_saver.get_state().node_storage.general_from_fast(*fast);
                            ui.label(format!("[{:0>2} {:0>2}]", identifier.0.x, identifier.0.y));

                            for (in_node_tree_node_index, req_at_index) in reqs_at.iter() {
                                ui.label(format!("In Node Index {in_node_tree_node_index} Req_at {req_at_index}"));
                            }
                        }
                        
                        
                    } else {
                        div(ui, |ui| {
                            ui.label("Out of bounds");
                        });
                    }
                });
            });

            egui::SidePanel::new(Side::Right, Id::new("Side Panel 2")).show(ctx, |ui| {
                puffin::profile_scope!("Search Tree");
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.set_min_width(200.0);

                    let grid = &self.state_saver.get_state().node_storage;
                    let depth_tree_controller = &self.state_saver.get_state().depth_tree_controller;
                    
                    let tree_identifiers = if self.selector.last_selected.is_some() {
                        let fast_identifier = grid.fast_from_general(GlobalPos(self.selector.last_selected.unwrap()));
                        let node = grid.get_node(fast_identifier);
                        node.tree_nodes.to_owned()
                    } else {
                        vec![]
                    };
                    
                    for (i, node) in depth_tree_controller.nodes.iter().enumerate() {
                        ui.separator();
                        
                        let selected = tree_identifiers.iter().find(|(_, test_i)| i == *test_i).is_some();
                        
                        let processed = if node.build {"D"} else {""};
                        let satisfied = if node.satisfied {"S"} else {""};
                        let level = node.level;
                        let pos = grid.general_from_fast(node.fast_identifier);
                        if selected {
                            ui.heading(RichText::new(format!(">>> Tree Node: {i} {level} {satisfied}{processed}")).strong());
                        } else {
                            ui.heading(format!("Tree Node: {i} {level} {satisfied}{processed}"));
                        }
                        
                        ui.label(format!("Pos: [{:0>2},{:0>2}]", pos.0.x, pos.0.y));
                        ui.label(format!("Value: {:?}", node.value_data.value_type));
                        
                        ui.label("Reqs:");
                        for req_at in node.reqs.iter() {
                            ui.separator();
                            
                            let req_pos = grid.general_from_fast(req_at.fast_identifier);
                            ui.label(format!("ReqPos: [{},{}]", req_pos.0.x, req_pos.0.y));
                            for (i, (req_value_nr, tree_index)) in req_at.tree_nodes.iter().enumerate() {
                                let value_type = req_value_nr.value_type;

                                ui.label(format!("{value_type:?} at Tree Index {tree_index}"));
                            }
                        }

                        ui.label("Req by:");
                        for (tree_index, req_at_index, in_req_at_index) in node.req_by.iter() {
                            ui.separator();

                            ui.label(format!("Tree Node {tree_index} req_at index {req_at_index} req index {in_req_at_index}"));
                        }
                    }
                });
            });
            
            egui::CentralPanel::default().show(ctx, |ui| {
                puffin::profile_scope!("Center Panel");

                let available_size = egui_vec2_to_glam_vec2(ui.available_size());
                self.grid_renderer.wanted_size = available_size.as_uvec2();


                if let Some(Pos2{x, y}) = ui.ctx().pointer_latest_pos() {
                    self.pointer_pos_in_grid = Some(vec2(
                        x - ui.next_widget_position().x,
                        y - ui.next_widget_position().y
                    ));
                } else {
                    self.pointer_pos_in_grid = None;
                }

                let image = self.grid_renderer.get_egui_image(frame_index);
                if image.is_some() {
                    ui.add(image.unwrap());
                }
            });
        })?;

        command_buffer.end_rendering();

        command_buffer.swapchain_image_render_barrier(swap_chain_image)?;

        Ok(())
    }

    pub fn on_window_event(&mut self, base: &mut BaseApp<LazyModelSynthesis>, event: &WindowEvent) -> Result<()> {
        self.gui.handle_event(&base.window, event);

        Ok(())
    }
}

fn egui_vec2_to_glam_vec2(v: egui::Vec2) -> glam::Vec2 {
    vec2(v.x, v.y)
}

fn div(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none().show(ui, |ui| {
        ui.with_layout(Layout::left_to_right(Align::TOP), add_contents);
    });
}

fn div_vert(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none().show(ui, |ui| {
        ui.with_layout(Layout::top_down(Align::LEFT), add_contents);
    });
}
