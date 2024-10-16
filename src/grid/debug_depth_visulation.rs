use std::ffi::c_float;
use crate::util::state_saver::TickType;
use std::time::Duration;
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
use crate::dispatcher::Dispatcher;
use crate::grid::grid::{Grid, ValueData};
use crate::grid::identifier::{ChunkNodeIndex, GlobalPos, PackedChunkNodeIndex};
use crate::dispatcher::random_dispatcher::RandomDispatcher;
use crate::go_back_in_time::node_manager::GoBackNodeManager;
use crate::grid::render::node_render_data::NUM_VALUE_TYPES;
use crate::grid::render::renderer::GridRenderer;
use crate::grid::render::selector::Selector;
use crate::grid::rules::{get_example_rules, NUM_REQS, NUM_VALUES, ValueType};
use crate::LazyModelSynthesis;
use crate::general_data_structure::identifier::IdentifierConverterT;
use crate::general_data_structure::node_storage::NodeStorageT;
use crate::util::state_saver::StateSaver;
use crate::general_data_structure::{ValueDataT, ValueNr};

const CHUNK_SIZE: usize = 32;
const DEBUG_MODE: bool = true;

pub struct GridDebugDepthVisulation {
    pub gui: Gui,
    
    pub state_saver: StateSaver<
        DepthNodeManager<
            Grid<DepthNode<ValueData>>, 
            RandomDispatcher<ChunkNodeIndex>,
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
    pointer_pos_in_grid: Option<Vec2>
}

impl GridDebugDepthVisulation {
    pub fn new(base: &mut BaseApp<LazyModelSynthesis>) -> Result<Self> {

        let mut grid = Grid::new(CHUNK_SIZE);
        grid.add_chunk(IVec2::ZERO);
        grid.rules = get_example_rules();

        let mut node_manager = DepthNodeManager::new(grid, NUM_VALUES, NUM_REQS);
        node_manager.select_value(GlobalPos(IVec2::new(0, 0)), ValueData::new(ValueType::Stone));
        
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
        
        if self.run || DEBUG_MODE {
            self.state_saver.set_next_tick(TickType::ForwardSave);
            for _ in 0..self.run_ticks_per_frame {
                self.state_saver.tick();
            }
        } else {
            self.state_saver.tick();
        }
        self.state_saver.set_next_tick(TickType::None);
        
        
        if DEBUG_MODE {
            let mut d = self.state_saver.get_state().dispatcher.clone();
            if d.pop_add().is_none() && d.pop_remove().is_none() && d.pop_select().is_none() {
                let gi = GlobalPos(ivec2(10, 10));
                let fi = self.state_saver.get_state().node_storage.fast_from_general(gi);
                let node = self.state_saver.get_state().node_storage.get_node(fi);
                let v = &node.values[0];
                let vt = ValueType::try_from_primitive(v.value_data.get_value_nr()).unwrap();
                let next_vt = if vt == ValueType::Grass {
                    ValueType::Stone
                } else {
                    ValueType::Grass
                };
                
                self.state_saver.get_state_mut().select_value(gi, ValueData::new(next_vt));
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
                        ui.heading("Grid (Debug Mode)");
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
                            let value_nr = i as ValueNr;
                            let value_type = ValueType::try_from_primitive(value_nr).unwrap();
                            
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
                        
                        for i in 0..NUM_VALUE_TYPES {
                            let value_nr = i as ValueNr;
                            
                            let added = data.get_value_type(value_nr);
                            let add_queue = data.get_add_queue(value_nr);
                            let propergate_queue = data.get_remove_queue(value_nr);
                            let reset_queue = data.get_select_queue();
                            
                            div(ui, |ui| {
                                ui.label(
                                    format!(
                                        "{} {:?} {} {} {}",
                                        if added {"x"} else {"   "},
                                        ValueType::try_from_primitive(i).unwrap(),
                                        if add_queue {"A"} else {"   "},
                                        if propergate_queue {"R"} else {"   "},
                                        if reset_queue {"S"} else {"   "},
                                    ));
                            });


                        }
                        
                    } else {
                        div(ui, |ui| {
                            ui.label("Out of bounds");
                        });
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
