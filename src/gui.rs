use egui::{Color32, SliderOrientation, Ui, Vec2};
use egui::style::Spacing;
use egui::Label;

use client_server::GroupToRotate;

use std::fs::File;
use std::string::String;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};
use std::str;

use serde::__private::de::Content::{String as OtherString};
use serde::de::Unexpected::Option;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {

    #[serde(skip)]
    stream: TcpStream, //Option<TcpStream>,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    corner_a: f32,
    corner_b: f32,
    corner_c: f32,
    corner_d: f32,

    check_box: bool,
    is_connected_user: bool,

    login_str: String,
    password_str: String,
    message_to_user: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            corner_a: 0.0,
            corner_b: 0.0,
            corner_c: 0.0,
            corner_d: 0.0,
            check_box: false,
            login_str: String::new(),
            password_str: String::new(),
            message_to_user: String::new(),
            is_connected_user: false,
            stream: TcpStream::connect("127.0.0.1:7878").unwrap(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            corner_a,
            corner_b,
            corner_c,
            corner_d,
            check_box,
            login_str,
            password_str,
            message_to_user,
            is_connected_user,
            stream,
        } = self;

        let mut output = File::create("current_corners.txt").unwrap();
        write!(output, "{}\n{}\n{}\n{}", corner_a, corner_b, corner_c, corner_d).unwrap();

        if (*check_box) && (*is_connected_user){
            stream.write(format!("MOVE_{corner_a}_{corner_b}_{corner_c}_{corner_d}").as_bytes()).expect("failed to write");

            let mut reader = BufReader::new(&*stream);
            let mut buffer: Vec<u8> = Vec::new();

            reader.read_until(b'\n',&mut buffer).unwrap();
        };

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Server");

            ui.spacing_mut().button_padding = Vec2::new(15.0, 5.0);

            ui.checkbox(check_box, "Real Time");

            if *check_box == false {
                ui.horizontal(|ui| {
                    if (ui.button("MOVE").clicked()) && (*is_connected_user) {
                        stream.write(format!("MOVE_{corner_a}_{corner_b}_{corner_c}_{corner_d}").as_bytes()).expect("failed to write");

                        let mut reader = BufReader::new(&*stream);
                        let mut buffer: Vec<u8> = Vec::new();
                        reader.read_until(b'\n',&mut buffer).unwrap();

                        let server_answer = str::from_utf8(&buffer).unwrap();
                        match server_answer {
                            "MOVED\n" => *message_to_user = String::from("Successfully moved"),
                            _ => *message_to_user = String::from("Error with move"),
                        }
                    }
                    if ui.button("STOP").clicked() {
                        //pass
                    }
                });
                if (ui.button("READ POSITION").clicked()) && (*is_connected_user) {
                    stream.write("READ_POSITION".as_bytes());

                    let mut reader = BufReader::new(&*stream);
                    let mut buffer: Vec<u8> = Vec::new();
                    reader.read_until(b'\n',&mut buffer).unwrap();

                    let server_answer = str::from_utf8(&buffer).unwrap();

                    if &server_answer[0..8] == "POSITION" {
                        let vec: Vec<&str> = server_answer.split("_").collect();
                        *corner_a = vec[1].parse::<f32>().unwrap();
                        *corner_b = vec[2].parse::<f32>().unwrap();
                        *corner_c = vec[3].parse::<f32>().unwrap();
                        *corner_d = vec[4][..3].parse::<f32>().unwrap();
                        *message_to_user = String::from("Position successfully read from server");
                    } else {*message_to_user = String::from("Error with reading position")}
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {

                if ui.button("Log in").clicked() {

                    stream.write(format!("LOGIN_{login_str}_{password_str}").as_bytes()).expect("failed to write");

                    let mut reader = BufReader::new(&*stream);
                    let mut buffer: Vec<u8> = Vec::new();
                    reader.read_until(b'\n',&mut buffer).unwrap();

                    let server_answer = str::from_utf8(&buffer).unwrap();
                    match server_answer {
                        "LOGGED_IN\n" => {
                            *message_to_user = String::from("You successfully logged in");
                            *login_str = String::from("");
                            *password_str = String::from("");
                            *is_connected_user = true;
                        },
                        "NO_USER\n" => *message_to_user = String::from("User does not exist"),
                        _ => *message_to_user = String::from("Error with logging in"),
                    }
                };

                if *is_connected_user {
                    ui.label("");
                } else {
                    ui.colored_label(Color32::from_rgb(255,0,0), "You are not logged in");
                }

                ui.horizontal(|ui| {
                    ui.label("Password: ");
                    ui.text_edit_singleline(password_str);
                });

                ui.horizontal(|ui| {
                    ui.label("Login: ");
                    ui.text_edit_singleline(login_str);
                });

                ui.label(format!("{message_to_user}"));

            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Axes of rotation");

            ui.spacing_mut().slider_width = 200.0;
            ui.spacing_mut().button_padding = Vec2::new(15.0, 5.0);

            ui.add( egui::Slider::new(corner_a, -1.5..=1.5).text("Axis A"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_a = 1.5;

                }
                if ui.button("MIN").clicked() {
                    *corner_a = -1.5;

                }
                if ui.button("+").clicked() {
                    *corner_a += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_a -= 0.1;
                }
            });
            ui.add( egui::Slider::new(corner_b, -1.5..=1.5).text("Axis B"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_b = 1.5;

                }
                if ui.button("MIN").clicked() {
                    *corner_b = -1.5;

                }
                if ui.button("+").clicked() {
                    *corner_b += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_b -= 0.1;
                }
            });
            ui.add( egui::Slider::new(corner_c, -1.5..=1.5).text("Axis C"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_c = 1.5;

                }
                if ui.button("MIN").clicked() {
                    *corner_c = -1.5;

                }
                if ui.button("+").clicked() {
                    *corner_c += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_c -= 0.1;
                }
            });
            ui.add( egui::Slider::new(corner_d, -3.3..=3.3).text("Axis D"));
            ui.horizontal(|ui| {
                if ui.button("MAX").clicked() {
                    *corner_d = 3.3;

                }
                if ui.button("MIN").clicked() {
                    *corner_d = -3.3;

                }
                if ui.button("+").clicked() {
                    *corner_d += 0.1;

                }
                if ui.button("-").clicked() {
                    *corner_d -= 0.1;
                }
            });
        });
    }
}