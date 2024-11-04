use crate::grid::{get_node_index_from_pos, Grid};
use crate::util::state_saver::TickType;
use std::time::Duration;
use octa_force::gui::Gui;
use octa_force::anyhow::*;
use octa_force::{BaseApp, egui, glam};
use octa_force::egui::{Align, FontId, Frame, Id, Layout, Pos2, RichText, Ui, Widget};
use octa_force::egui::FontFamily::Proportional;
use octa_force::egui::panel::Side;
use octa_force::egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::{ivec2, vec2, Vec2};
use octa_force::puffin_egui::puffin;
use octa_force::vulkan::ash::vk::AttachmentLoadOp;
use crate::grid_manager::GridManager;
use crate::render::node_render_data::NUM_VALUE_TYPES;
use crate::render::renderer::GridRenderer;
use crate::render::selector::Selector;
use crate::LazyModelSynthesis;
use crate::rules::ValueType;
use crate::util::state_saver::StateSaver;

pub const GRID_SIZE: usize = 32;
const DEBUG_MODE: bool = true;

pub struct Visualization {
    pub gui: Gui,
    
    pub state_saver: StateSaver<GridManager>,

    pub grid_renderer: GridRenderer,
    pub selector: Selector,

    run: bool,
    run_ticks_per_frame: usize,
    pointer_pos_in_grid: Option<Vec2>,
}

impl Visualization {
    pub fn new(base: &mut BaseApp<LazyModelSynthesis>) -> Result<Self> {

        let grid = Grid::new(Some(ValueType::Stone));
        
        let grid_manager = GridManager::new(grid);
        
        let state_saver = StateSaver::from_state(grid_manager, 100);

        let mut gui = Gui::new(
            &base.context,
            base.swapchain.format,
            base.swapchain.depth_format,
            &base.window,
            base.num_frames
        )?;

        let grid_renderer = GridRenderer::new(&mut base.context, &mut gui.renderer, base.num_frames, GRID_SIZE, 1)?;
        let selector = Selector::new();
        
        let mut v = Visualization {
            state_saver,
            gui,
            grid_renderer,
            selector,
            run: false,
            run_ticks_per_frame: 10,
            pointer_pos_in_grid: None,
        };
        v.place_random_value();

        Ok(v)
    }

    pub fn update(
        &mut self,
        base: &mut BaseApp<LazyModelSynthesis>,
        frame_index: usize,
        _delta_time: Duration,
    ) -> Result<()> {
        
        if base.controls.mouse_left && self.selector.last_selected.is_some() && self.selector.value_type_to_place.is_some() {
            self.state_saver.get_state_mut().select_value(
                self.selector.last_selected.unwrap(), 
                self.selector.value_type_to_place.unwrap()
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
        
        self.selector.add_to_render_data(self.pointer_pos_in_grid, &mut self.state_saver.get_state_mut().grid);

        
        let working_grids = &self.state_saver.get_state().working_grids;
        if !working_grids.is_empty() {
            self.grid_renderer.set_chunk_data(0, GRID_SIZE, &working_grids[0].full_grid.render_data);
        } else {
            self.grid_renderer.set_chunk_data(0, GRID_SIZE, &self.state_saver.get_state().grid.render_data);
        }
        
        self.grid_renderer.update(&mut base.context, base.swapchain.format, frame_index);

        self.selector.clear_from_render_data(&mut self.state_saver.get_state_mut().grid);
        
        Ok(())
    }
    
    fn place_random_value(&mut self) {
        let pos = ivec2(fastrand::i32(0..32), fastrand::i32(0..32));
        let node_index = get_node_index_from_pos(pos);
        let node = self.state_saver.get_state().grid.nodes[node_index];
        let v = &node.value;
        let next_vt = if v.is_some() {
            let vt = v.unwrap();
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

        self.state_saver.get_state_mut().select_value(pos, next_vt);

        self.run = false;
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
                            let value_type = ValueType::from_value_nr(i);
                            
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

fn div_vert(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none().show(ui, |ui| {
        ui.with_layout(Layout::top_down(Align::LEFT), add_contents);
    });
}
