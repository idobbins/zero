use leptos::prelude::*;
use web_sys::{MouseEvent, WheelEvent};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    fn distance_to(&self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Clone)]
struct DraggableBox {
    id: u32,
    position: Point,
    size: Point,
    color: String,
}

#[derive(Clone, Copy, Debug)]
struct Viewport {
    pan: Point,
    zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            pan: Point::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
enum DragState {
    None,
    MouseDown { 
        start_pos: Point, 
        target: DragTarget,
        has_moved: bool,
    },
    Dragging { 
        target: DragTarget,
        start_pos: Point,
        start_data: DragData,
    },
}

#[derive(Clone, Debug)]
enum DragTarget {
    Background,
    Box(u32),
}

#[derive(Clone, Debug)]
enum DragData {
    ViewportPan(Point),
    BoxPosition(Point),
}

impl Default for DragState {
    fn default() -> Self {
        Self::None
    }
}

const DRAG_THRESHOLD: f64 = 3.0; // pixels

fn screen_to_world_coords(screen_pos: Point, viewport: Viewport) -> Point {
    Point::new(
        (screen_pos.x - viewport.pan.x) / viewport.zoom,
        (screen_pos.y - viewport.pan.y) / viewport.zoom,
    )
}

fn point_in_box(point: Point, box_pos: Point, box_size: Point) -> bool {
    point.x >= box_pos.x 
        && point.x <= box_pos.x + box_size.x
        && point.y >= box_pos.y 
        && point.y <= box_pos.y + box_size.y
}

fn generate_color(id: u32) -> String {
    let colors = ["#FF6B6B", "#4ECDC4", "#45B7D1", "#96CEB4", "#FFEAA7", "#DDA0DD", "#98D8C8"];
    colors[id as usize % colors.len()].to_string()
}

#[component]
fn App() -> impl IntoView {
    let (boxes, set_boxes) = signal(vec![
        DraggableBox {
            id: 0,
            position: Point::new(100.0, 100.0),
            size: Point::new(80.0, 60.0),
            color: generate_color(0),
        },
        DraggableBox {
            id: 1,
            position: Point::new(250.0, 150.0),
            size: Point::new(100.0, 80.0),
            color: generate_color(1),
        },
    ]);
    let (next_id, set_next_id) = signal(2);
    let (viewport, set_viewport) = signal(Viewport::default());
    let (drag_state, set_drag_state) = signal(DragState::default());

    // Store global event listeners
    let (global_listeners_active, set_global_listeners_active) = signal(false);

    // Create the global mouse move handler
    let create_global_mouse_move = move || {
        let drag_state = drag_state.clone();
        let viewport = viewport.clone();
        let set_viewport = set_viewport.clone();
        let boxes = boxes.clone();
        let set_boxes = set_boxes.clone();
        let set_drag_state = set_drag_state.clone();
        
        Closure::wrap(Box::new(move |ev: web_sys::MouseEvent| {
            let current_pos = Point::new(ev.client_x() as f64, ev.client_y() as f64);
            
            match drag_state.get() {
                DragState::MouseDown { start_pos, target, has_moved } => {
                    let distance = start_pos.distance_to(current_pos);
                    
                    if distance > DRAG_THRESHOLD {
                        // Start actual dragging
                        let drag_data = match target {
                            DragTarget::Background => DragData::ViewportPan(viewport.get().pan),
                            DragTarget::Box(box_id) => {
                                if let Some(draggable_box) = boxes.get().iter().find(|b| b.id == box_id) {
                                    DragData::BoxPosition(draggable_box.position)
                                } else {
                                    return; // Box not found, abort
                                }
                            }
                        };
                        
                        set_drag_state.set(DragState::Dragging {
                            target,
                            start_pos,
                            start_data: drag_data,
                        });
                    } else if !has_moved {
                        // Update that mouse has moved (but not enough to start dragging)
                        set_drag_state.set(DragState::MouseDown {
                            start_pos,
                            target,
                            has_moved: true,
                        });
                    }
                }
                DragState::Dragging { target, start_pos, start_data } => {
                    let delta = Point::new(
                        current_pos.x - start_pos.x,
                        current_pos.y - start_pos.y,
                    );
                    
                    match (target, start_data) {
                        (DragTarget::Background, DragData::ViewportPan(start_pan)) => {
                            set_viewport.update(|v| {
                                v.pan = Point::new(
                                    start_pan.x + delta.x,
                                    start_pan.y + delta.y,
                                );
                            });
                        }
                        (DragTarget::Box(box_id), DragData::BoxPosition(start_position)) => {
                            // Convert screen delta to world delta
                            let delta_world = Point::new(
                                delta.x / viewport.get().zoom,
                                delta.y / viewport.get().zoom,
                            );
                            
                            let new_position = Point::new(
                                start_position.x + delta_world.x,
                                start_position.y + delta_world.y,
                            );
                            
                            set_boxes.update(|boxes| {
                                if let Some(draggable_box) = boxes.iter_mut().find(|b| b.id == box_id) {
                                    draggable_box.position = new_position;
                                }
                            });
                        }
                        _ => {} // Invalid combination, ignore
                    }
                }
                DragState::None => {}
            }
        }) as Box<dyn FnMut(_)>)
    };

    // Create the global mouse up handler
    let create_global_mouse_up = move || {
        let drag_state = drag_state.clone();
        let set_drag_state = set_drag_state.clone();
        let viewport = viewport.clone();
        let boxes = boxes.clone();
        let set_boxes = set_boxes.clone();
        let next_id = next_id.clone();
        let set_next_id = set_next_id.clone();
        let set_global_listeners_active = set_global_listeners_active.clone();
        
        Closure::wrap(Box::new(move |ev: web_sys::MouseEvent| {
            let current_pos = Point::new(ev.client_x() as f64, ev.client_y() as f64);
            
            match drag_state.get() {
                DragState::MouseDown { start_pos: _, target, has_moved: _ } => {
                    // This was a click (mouse down + up without significant movement)
                    match target {
                        DragTarget::Background => {
                            // Create a new box at the click location
                            let world_pos = screen_to_world_coords(current_pos, viewport.get());
                            let new_box = DraggableBox {
                                id: next_id.get(),
                                position: Point::new(world_pos.x - 40.0, world_pos.y - 30.0),
                                size: Point::new(80.0, 60.0),
                                color: generate_color(next_id.get()),
                            };
                            set_boxes.update(|bs| bs.push(new_box));
                            set_next_id.update(|n| *n += 1);
                        }
                        DragTarget::Box(_box_id) => {
                            // Just a click on a box - could add selection logic here
                            // For now, do nothing
                        }
                    }
                }
                DragState::Dragging { .. } => {
                    // Dragging finished - no additional action needed
                }
                DragState::None => {
                    // Shouldn't happen, but handle gracefully
                }
            }
            
            // Reset drag state and remove global listeners
            set_drag_state.set(DragState::None);
            set_global_listeners_active.set(false);
        }) as Box<dyn FnMut(_)>)
    };

    // Effect to manage global event listeners
    create_effect(move |_| {
        let current_drag_state = drag_state.get();
        let should_have_listeners = !matches!(current_drag_state, DragState::None);
        let currently_active = global_listeners_active.get();
        
        if should_have_listeners && !currently_active {
            // Add global listeners
            let document = web_sys::window().unwrap().document().unwrap();
            let mouse_move_closure = create_global_mouse_move();
            let mouse_up_closure = create_global_mouse_up();
            
            let _ = document.add_event_listener_with_callback(
                "mousemove",
                mouse_move_closure.as_ref().unchecked_ref()
            );
            let _ = document.add_event_listener_with_callback(
                "mouseup",
                mouse_up_closure.as_ref().unchecked_ref()
            );
            
            // Store closures to prevent them from being dropped
            mouse_move_closure.forget();
            mouse_up_closure.forget();
            
            set_global_listeners_active.set(true);
        } else if !should_have_listeners && currently_active {
            // Remove global listeners - they'll be cleaned up automatically when closures are dropped
            set_global_listeners_active.set(false);
        }
    });

    let handle_mouse_down = move |ev: MouseEvent| {
        ev.prevent_default();
        
        let screen_pos = Point::new(ev.client_x() as f64, ev.client_y() as f64);
        let world_pos = screen_to_world_coords(screen_pos, viewport.get());
        
        // Check if we clicked on a box (check in reverse order for top-most)
        let mut target = DragTarget::Background;
        for draggable_box in boxes.get().iter().rev() {
            if point_in_box(world_pos, draggable_box.position, draggable_box.size) {
                target = DragTarget::Box(draggable_box.id);
                break;
            }
        }
        
        set_drag_state.set(DragState::MouseDown {
            start_pos: screen_pos,
            target,
            has_moved: false,
        });
    };

    let handle_wheel = move |ev: WheelEvent| {
        ev.prevent_default();
        
        let mouse_pos = Point::new(ev.client_x() as f64, ev.client_y() as f64);
        let zoom_factor = if ev.delta_y() > 0.0 { 0.9 } else { 1.1 };
        
        set_viewport.update(|v| {
            let old_zoom = v.zoom;
            v.zoom = (v.zoom * zoom_factor).clamp(0.1, 3.0);
            let zoom_change = v.zoom / old_zoom;
            
            // Adjust pan to zoom towards mouse position
            v.pan.x = mouse_pos.x - (mouse_pos.x - v.pan.x) * zoom_change;
            v.pan.y = mouse_pos.y - (mouse_pos.y - v.pan.y) * zoom_change;
        });
    };

    // Handle box-specific mouse events to prevent event bubbling
    let handle_box_mouse_down = move |ev: MouseEvent, box_id: u32| {
        ev.stop_propagation(); // Prevent bubbling to parent
        ev.prevent_default();
        
        let screen_pos = Point::new(ev.client_x() as f64, ev.client_y() as f64);
        
        set_drag_state.set(DragState::MouseDown {
            start_pos: screen_pos,
            target: DragTarget::Box(box_id),
            has_moved: false,
        });
    };

    view! {
        <div 
            style="
                width: 100vw; 
                height: 100vh; 
                overflow: hidden; 
                cursor: grab; 
                position: relative;
                background: linear-gradient(45deg, #f0f0f0 25%, transparent 25%), 
                           linear-gradient(-45deg, #f0f0f0 25%, transparent 25%), 
                           linear-gradient(45deg, transparent 75%, #f0f0f0 75%), 
                           linear-gradient(-45deg, transparent 75%, #f0f0f0 75%);
                background-size: 20px 20px;
                background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
            "
            on:mousedown=handle_mouse_down
            on:wheel=handle_wheel
        >
            // Container for all draggable elements with viewport transform
            <div 
                style=move || {
                    let v = viewport.get();
                    format!(
                        "transform: translate({}px, {}px) scale({}); transform-origin: 0 0; position: absolute;",
                        v.pan.x, v.pan.y, v.zoom
                    )
                }
            >
                <For
                    each=move || boxes.get()
                    key=|draggable_box| draggable_box.id
                    children=move |draggable_box| {
                        let box_id = draggable_box.id;
                        view! {
                            <div 
                                style=move || {
                                    let is_dragging = matches!(
                                        drag_state.get(),
                                        DragState::Dragging { target: DragTarget::Box(id), .. } if id == box_id
                                    );
                                    let cursor = if is_dragging { "grabbing" } else { "grab" };
                                    let shadow = if is_dragging { "0 8px 16px rgba(0,0,0,0.3)" } else { "0 4px 8px rgba(0,0,0,0.2)" };
                                    
                                    format!(
                                        "
                                        position: absolute;
                                        left: {}px;
                                        top: {}px;
                                        width: {}px;
                                        height: {}px;
                                        background-color: {};
                                        border: 2px solid #333;
                                        border-radius: 8px;
                                        cursor: {};
                                        display: flex;
                                        align-items: center;
                                        justify-content: center;
                                        font-weight: bold;
                                        color: white;
                                        text-shadow: 1px 1px 2px rgba(0,0,0,0.7);
                                        box-shadow: {};
                                        user-select: none;
                                        transition: box-shadow 0.2s ease;
                                        z-index: {};
                                        ",
                                        draggable_box.position.x,
                                        draggable_box.position.y,
                                        draggable_box.size.x,
                                        draggable_box.size.y,
                                        draggable_box.color,
                                        cursor,
                                        shadow,
                                        if is_dragging { 1000 } else { 1 }
                                    )
                                }
                                on:mousedown=move |ev| handle_box_mouse_down(ev, box_id)
                            >
                                {format!("Box {}", draggable_box.id)}
                            </div>
                        }
                    }
                />
            </div>
            
            // Instructions overlay
            <div style="
                position: absolute;
                top: 10px;
                left: 10px;
                background: rgba(0,0,0,0.8);
                color: white;
                padding: 10px;
                border-radius: 5px;
                font-family: monospace;
                font-size: 12px;
                pointer-events: none;
                z-index: 1000;
            ">
                "🖱️ Click empty space: Create box" <br/>
                "🖱️ Drag box: Move box" <br/>
                "🖱️ Drag background: Pan view" <br/>
                "🖱️ Scroll: Zoom in/out" <br/>
                <br/>
                {move || format!("State: {:?}", drag_state.get())} <br/>
                {move || format!("Global listeners: {}", global_listeners_active.get())}
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
