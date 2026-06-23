use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
// 1. Общие состояния для ЛЮБОГО элемента интерфейса
#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    pub is_visible: bool,
    pub is_hovered: bool,  // Мышка наведена (ваш highlight)
    pub is_focused: bool,  // Элемент активен (для ввода текста)
    pub is_pressed: bool,  // Зажата кнопка мыши
    pub is_disabled: bool, // Серый/неактивный элемент
    pub is_dragdrop: bool,
    pub is_move: bool,
}
#[derive(Debug, Clone, Default)]
pub struct Tooltip {
    pub text: String,
}
#[derive(Debug, Clone)]
pub struct WidgetStyle {
    pub sprite: String,
}
#[derive(Debug, Clone)]
pub struct ItemSlotStyle {
    pub normal_sprite: String,
    pub hover_sprite: String,
}
#[derive(Debug, Clone)]
pub struct TextFieldStyle {
    pub color: u32,
}
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub normal_sprite: String,
    pub hover_sprite: String,
    pub pressed_sprite: String,
    pub disabled_sprite: String,
}
#[derive(Debug, Clone)]
pub struct TabStyle {
    pub normal_sprite: String,
    pub hover_sprite: String,
    pub pressed_sprite: String,
    pub disabled_sprite: String,
}
#[derive(Debug, Clone)]
pub struct SliderStyle {
    pub bg: String,
    pub knob: String,
    pub normal_knob: String,
    pub hover_knob: String,
    pub pressed_knob: String,
    pub disabled_knob: String,
}
#[derive(Debug, Clone)]
pub struct RadioButtonStyle {
    pub bg: String,
    pub activator: String,
}
#[derive(Debug, Clone)]
pub struct CheckBoxStyle {
    pub bg: String,
    pub activator: String,
}
#[derive(Debug, Clone)]
pub struct ActivateStyle {
    pub active_normal_sprite: String,
    pub disactive_normal_sprite: String,
    pub hover_sprite: String,
    pub pressed_sprite: String,
    pub disabled_sprite: String,
}
#[derive(Debug, Clone)]
pub struct DropDownStyle {
    pub bg_sprite: String,
}
#[derive(Debug, Clone)]
pub struct LoadingBarStyle {
    pub bg_sprite: String,
    pub fill_sprite: String,
}
//
// 2. Только уникальные данные конкретного виджета
#[derive(Debug, Clone)]
pub enum WidgetKind {
    ItemSlot {
        item_count: u32,
        item_id: u32,
        style: ItemSlotStyle,
    },
    TextField {
        text: String,
        wrap: bool,
        style: TextFieldStyle,
    },
    Button {
        text: String,
        style: ButtonStyle,
    },
    Tab {
        text: String,
        is_active: bool, // Активна ли вкладка прямо сейчас
        style: TabStyle,
    },
    Slider {
        value: f32,
        min: f32,
        max: f32,
        style: SliderStyle,
    },
    RadioButton {
        is_selected: bool,
        style: RadioButtonStyle,
    },
    CheckBox {
        is_selected: bool,
        style: CheckBoxStyle,
    },
    Activate {
        is_activate: bool,
        style: ActivateStyle,
    },
    DropDown {
        is_open: bool,
        text: String,
        select_index: u32,
        container: Vec<String>,
        style: DropDownStyle,
    },
    LoadingBar {
        progress: f32,
        text: String,
        style: LoadingBarStyle,
    },
    Container {
        children: Vec<Widget>,
        scroll_y: f32,
    },
}

// 1. Все возможные действия, которые могут произойти в UI
#[derive(Debug, Clone)]
pub enum UiEvent {
    ButtonClicked {
        widget_id: u64,
    },
    DropDownChanged {
        widget_id: u64,
        new_index: u32,
    },
    SliderMoved {
        widget_id: u64,
        new_value: f32,
    },
    ActivateToggled {
        widget_id: u64,
        is_activate: bool,
    },
    DragDrop {
        from_widget_id: u64,
        to_widget_id: u64,
    },
    MoveWidget {
        widget_id: u64,
        new_x: f32,
        new_y: f32,
    },
    InputText {
        widget_id: u64,
        text: String,
    },
}

// 3. Финальный виджет
#[derive(Debug, Clone)]
pub struct Widget {
    pub id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub style: WidgetStyle,
    pub state: WidgetState, // Состояние взаимодействия
    pub tooltip: Tooltip,
    pub kind: WidgetKind, // Суть виджета
}

fn parser(tokens: &mut std::str::SplitWhitespace<'_>, current_node: &mut i32, container: &mut i32) {
    while let Some(token) = tokens.next() {
        if token.eq("{") {
            // break;
        } else if token.eq("}") || token.eq(";") {
            // break;
        } else {
            if token.eq("DesktopGame") {
                *current_node = 100;
            } else if token.eq("Window") {
                *current_node = 200;
            } else if token.eq("Button") {
                *current_node = 300;
            } else if token.eq("Text") {
                *current_node = 400;
            } else if token.eq("Style") {
                *current_node = 500;
            } else if token.eq("State") {
                *current_node = 600;
            } else if token.eq("Tooltip") {
                *current_node = 700;
            }
            match *current_node {
                100 => {
                    if let Some((key, val)) = token.split_once('=') {
                        match key {
                            "id" => println!("{}", val),
                            "name" => println!("{}", val),
                            "x" => println!("{}", val),
                            "y" => println!("{}", val),
                            "w" => println!("{}", val),
                            "h" => println!("{}", val),
                            "container" => {
                                println!("container {}", val);
                                let temp = val.parse().unwrap();
                                *container = temp;
                            }
                            "texture" => println!("{}", val),
                            "text" => println!("{}", val),
                            "normal" => println!("{}", val),
                            "pushed" => println!("{}", val),
                            "highlight" => println!("{}", val),
                            _ => {}
                        }
                    }
                }
                200 => {
                    if let Some((key, val)) = token.split_once('=') {
                        match key {
                            "id" => println!("{}", val),
                            "name" => println!("{}", val),
                            "x" => println!("{}", val),
                            "y" => println!("{}", val),
                            "w" => println!("{}", val),
                            "h" => println!("{}", val),
                            "container" => {
                                println!("container {}", val);
                                let temp = val.parse().unwrap();
                                *container = temp;
                                // println!("{}", *container);
                            }
                            "texture" => println!("{}", val),
                            "text" => println!("{}", val),
                            "normal" => println!("{}", val),
                            "pushed" => println!("{}", val),
                            "highlight" => println!("{}", val),
                            _ => {}
                        }
                    }
                }
                300 => {
                    if let Some((key, val)) = token.split_once('=') {
                        match key {
                            "id" => println!("{}", val),
                            "name" => println!("{}", val),
                            "x" => println!("{}", val),
                            "y" => println!("{}", val),
                            "w" => println!("{}", val),
                            "h" => println!("{}", val),
                            "container" => {
                                println!("container {}", val);
                                let temp = val.parse().unwrap();
                                *container = temp;
                                // println!("{}", *container);
                            }
                            "texture" => println!("{}", val),
                            "text" => println!("{}", val),
                            "normal" => println!("{}", val),
                            "pushed" => println!("{}", val),
                            "highlight" => println!("{}", val),
                            _ => {}
                        }
                    }
                }
                400 => {
                    if let Some((key, val)) = token.split_once('=') {
                        match key {
                            "id" => println!("{}", val),
                            "name" => println!("{}", val),
                            "x" => println!("{}", val),
                            "y" => println!("{}", val),
                            "w" => println!("{}", val),
                            "h" => println!("{}", val),
                            "container" => {
                                println!("container {}", val);
                                let temp = val.parse().unwrap();
                                *container = temp;
                                // println!("{}", *container);
                            }
                            "texture" => println!("{}", val),
                            "text" => println!("{}", val),
                            "normal" => println!("{}", val),
                            "pushed" => println!("{}", val),
                            "highlight" => println!("{}", val),
                            _ => {}
                        }
                    }
                }
                500 => {
                    if let Some((key, val)) = token.split_once('=') {
                        match key {
                            "id" => println!("{}", val),
                            "name" => println!("{}", val),
                            "x" => println!("{}", val),
                            "y" => println!("{}", val),
                            "w" => println!("{}", val),
                            "h" => println!("{}", val),
                            "container" => {
                                println!("container {}", val);
                                let temp = val.parse().unwrap();
                                *container = temp;
                                // println!("{}", *container);
                            }
                            "texture" => println!("{}", val),
                            "text" => println!("{}", val),
                            "normal" => println!("{}", val),
                            "pushed" => println!("{}", val),
                            "highlight" => println!("{}", val),
                            _ => {}
                        }
                    }
                }
                600 => {
                    if let Some((key, val)) = token.split_once('=') {
                        match key {
                            "id" => println!("{}", val),
                            "name" => println!("{}", val),
                            "x" => println!("{}", val),
                            "y" => println!("{}", val),
                            "w" => println!("{}", val),
                            "h" => println!("{}", val),
                            "container" => {
                                println!("container {}", val);
                                let temp = val.parse().unwrap();
                                *container = temp;
                                // println!("{}", *container);
                            }
                            "texture" => println!("{}", val),
                            "text" => println!("{}", val),
                            "normal" => println!("{}", val),
                            "pushed" => println!("{}", val),
                            "highlight" => println!("{}", val),
                            _ => {}
                        }
                    }
                }
                700 => {
                    if let Some((key, val)) = token.split_once('=') {
                        match key {
                            "id" => println!("{}", val),
                            "name" => println!("{}", val),
                            "x" => println!("{}", val),
                            "y" => println!("{}", val),
                            "w" => println!("{}", val),
                            "h" => println!("{}", val),
                            "container" => {
                                println!("container {}", val);
                                let temp = val.parse().unwrap();
                                *container = temp;
                                // println!("{}", *container);
                            }
                            "texture" => println!("{}", val),
                            "text" => println!("{}", val),
                            "normal" => println!("{}", val),
                            "pushed" => println!("{}", val),
                            "highlight" => println!("{}", val),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            let childrens = container.clone();
            // let mut children_list = Vec::new();
            for _ in 0..childrens {
                parser(tokens, current_node, container);
                // if let Some(child) = parser(tokens, current_node, container) {
                //     children_list.push(child);
                // }
            }
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("settingui.txt")
        .map_err(|e| e.to_string())
        .expect("msg");
    let formatted = content
        .replace("{", " { ")
        .replace("}", " } ")
        .replace(";", " ; ");
    let mut tokens = formatted.split_whitespace();
    let mut current_node = 0;
    let mut container = 0;
    parser(&mut tokens, &mut current_node, &mut container);
    // println!("{:?}", tokens);
}
