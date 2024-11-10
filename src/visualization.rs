use crate::grid::{get_node_index_from_pos, Grid};
use crate::util::state_saver::TickType;
use std::time::Duration;
use octa_force::gui::Gui;
use octa_force::anyhow::*;
use octa_force::{egui, glam, Engine};
use octa_force::egui::{Align, FontId, Frame, Id, Layout, Pos2, RichText, Ui, Widget};
use octa_force::egui::FontFamily::Proportional;
use octa_force::egui::panel::Side;
use octa_force::egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use octa_force::egui_winit::winit::event::WindowEvent;
use octa_force::glam::{ivec2, vec2, Vec2};
use octa_force::puffin_egui::puffin;
use octa_force::vulkan::ash::vk::AttachmentLoadOp;
use crate::grid_manager::GridManager;
use crate::render::renderer::GridRenderer;
use crate::render::selector::Selector;
use crate::rule_gen::gen_rules_from_image;
use crate::rules::Rule;
use crate::util::state_saver::StateSaver;
use crate::value::{Value, ValueColor};

pub const GRID_SIZE: usize = 32;
const DEBUG_MODE: bool = true;

pub struct Visualization {
    pub gui: Gui,
    
    pub state_saver: StateSaver<GridManager>,

    pub grid_renderer: GridRenderer,
    pub selector: Selector,
    
    value_colors: Vec<ValueColor>,

    run: bool,
    run_ticks_per_frame: usize,
    pointer_pos_in_grid: Option<Vec2>,
    current_working_grid: Option<usize>,
}

impl Visualization {
    pub fn new(engine: &mut Engine) -> Result<Self> {
        let (rules, value_colors) = gen_rules_from_image(
            "WaveFunctionCollapse/samples/Angular.png", 
            vec![
                ivec2(-1, -1),
                ivec2(-1, 0),
                ivec2(-1, 1),
                ivec2(0, -1),
                ivec2(0, 1),
                ivec2(1, -1),
                ivec2(1, 0),
                ivec2(1, 1),
            ])?;
        
        let grid = Grid::new(Value(1));
        
        let grid_manager = GridManager::new(grid, rules);
        
        let state_saver = StateSaver::from_state(grid_manager, 100);

        let mut gui = Gui::new(
            &engine.context,
            engine.swapchain.format,
            engine.swapchain.depth_format,
            &engine.window,
            engine.num_frames
        )?;

        let mut grid_renderer = GridRenderer::new(&mut engine.context, &mut gui.renderer, engine.num_frames, GRID_SIZE, 1)?;
        let selector = Selector::new();

        grid_renderer.set_value_colors(&value_colors);
        
        
        let v = Visualization {
            state_saver,
            gui,
            grid_renderer,
            selector,
            run: false,
            run_ticks_per_frame: 10,
            pointer_pos_in_grid: None,
            current_working_grid: None,
            value_colors,
        };
        //v.place_random_value();
        
        Ok(v)
    }

    pub fn update(
        &mut self,
        engine: &mut Engine,
        frame_index: usize,
        _delta_time: Duration,
    ) -> Result<()> {
        if engine.controls.mouse_left && self.selector.selected_pos.is_some() && self.selector.value_type_to_place.is_some() {
            self.state_saver.get_state_mut().select_value(
                self.selector.selected_pos.unwrap(),
                self.selector.value_type_to_place
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
        
        
        if self.current_working_grid.is_some() && self.current_working_grid.unwrap() >= self.state_saver.get_state().working_grids.len() {
            self.current_working_grid = None;
        }

        
        let working_grids = &mut self.state_saver.get_state_mut().working_grids;
        if self.current_working_grid.is_some() {
            self.selector.set_selected_pos(self.pointer_pos_in_grid, &mut working_grids[self.current_working_grid.unwrap()].full_grid);
            
            
            self.grid_renderer.set_selector_pos(self.selector.selected_pos);
            self.grid_renderer.set_chunk_data(&working_grids[self.current_working_grid.unwrap()].full_grid.nodes);

            self.grid_renderer.update(&mut engine.context, engine.swapchain.format, frame_index);
            
             

            self.selector.clear_from_render_data(&mut working_grids[self.current_working_grid.unwrap()].full_grid);

        } else {
            self.selector.set_selected_pos(self.pointer_pos_in_grid, &mut self.state_saver.get_state_mut().grid);

            
            self.grid_renderer.set_selector_pos(self.selector.selected_pos);
            self.grid_renderer.set_chunk_data(&self.state_saver.get_state().grid.nodes);

            self.grid_renderer.update(&mut engine.context, engine.swapchain.format, frame_index);
            
             

            self.selector.clear_from_render_data(&mut self.state_saver.get_state_mut().grid);
        }
        
        /*
        if self.state_saver.get_state_mut().working_grids.is_empty() {
            self.place_random_value();
        }
        
         */

        Ok(())
    }
    
    /*
    fn place_random_value(&mut self) {
        let pos = ivec2(fastrand::i32(0..32), fastrand::i32(0..32));
        let node_index = get_node_index_from_pos(pos);
        let node = self.state_saver.get_state().grid.nodes[node_index];
        let v = &node.value;
        let next_vt = if v.is_some() {
            let vt = v.unwrap();
            if vt == Value::Stone {
                Value::Sand
            } else if vt == Value::Grass {
                Value::Stone
            } else {
                Value::Stone
            }
        } else {
            Value::Grass
        };

        self.state_saver.get_state_mut().select_value(pos, next_vt);
    }
    
     */

    pub fn record_render_commands(
        &mut self,
        engine: &mut Engine,
        frame_index: usize,
    ) -> Result<()> {

        let command_buffer = &engine.command_buffers[frame_index];

        let size = engine.swapchain.size;
        let swap_chain_image = &engine.swapchain.images_and_views[frame_index].image;
        let swap_chain_view = &engine.swapchain.images_and_views[frame_index].view;
        let swap_chain_depth_view = &engine.swapchain.depht_images_and_views[frame_index].view;

        self.grid_renderer.render(command_buffer, frame_index);

        command_buffer.begin_rendering(swap_chain_view, swap_chain_depth_view, size, AttachmentLoadOp::CLEAR, None);

        
        self.gui.cmd_draw(command_buffer, size, frame_index, &engine.window, &engine.context, |ctx| {
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

                        for i in 0..3 {
                            let value = Value::from_value_nr(i);
                            
                            let mut checked = self.selector.value_type_to_place == value; 
                            ui.checkbox(&mut checked, format!("{:?}", value));
                            if checked {
                                self.selector.value_type_to_place = value;
                            }
                        }
                        
                    });

                    ui.separator();

                    div(ui, |ui| {
                        ui.heading("Selected Node");
                    });

                    if let Some(pos) = self.selector.selected_pos {
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
            
            egui::SidePanel::new(Side::Right, Id::new("Right Panel")).show(ctx, |ui| {
                puffin::profile_scope!("Right Panel  ");

                ui.set_min_width(200.0);

                if ui.button("Unselect").clicked() {
                    self.current_working_grid = None;
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        for (i, working_grid) in self.state_saver.get_state().working_grids.iter().enumerate() {

                            let response = if self.current_working_grid == Some(i) {
                                ui.heading(format!("{i}: orders: {}", working_grid.orders.len()))
                            } else {
                                ui.label(format!("{i}: orders: {}", working_grid.orders.len()))
                            };

                            if response.hovered() {
                                self.current_working_grid = Some(i);
                            }
                        }
                    });
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

    pub fn on_window_event(&mut self, engine: &mut Engine, event: &WindowEvent) -> Result<()> {
        self.gui.handle_event(&engine.window, event);

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
