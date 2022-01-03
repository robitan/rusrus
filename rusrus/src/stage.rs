use std::time::Duration;

use yew::services::{ConsoleService, IntervalService, Task, KeyboardService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::keyboard::KeyListenerHandle;
use yew::events::{KeyboardEvent};

extern crate js_sys;

use yew::web_sys;

mod rus;
use rus::Model as Rus;
use rus::Color;
use rus::random_color;

pub struct Model {
    link: ComponentLink<Self>,
    job: Option<Box<dyn Task>>,
    event_listener: KeyListenerHandle,
    board: Vec<Vec<Color>>,
    movings: Option<(MovingRus, MovingRus)>,
    is_started: bool,
    is_gameover: bool,
}

#[derive(Debug, Clone)]
struct MovingRus {
    x: usize,
    y: usize,
    color: Color,
}

#[derive(Debug)]
pub enum Msg {
    Start,
    Fall,
    Erase,
    Generate,
    MovingsFall,
    GameOver,
    KeyboardEvent(KeyboardEvent),
}

const COLS:u32 = 8;
const ROWS:u32 = 12;
const MIN_ERASE_CHAIN:usize = 4;

const SPAWN_FALL_DERATION_MILLISEC:u64 = 1000;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let board:Vec<Vec<Color>> = ( 0..ROWS )
            .map(|_i| {
                ( 0..COLS ).map(|_i| {
                    random_color()
                } ).collect()
            }).collect();

        let event_listener = KeyboardService::register_key_press(
            &web_sys::window().unwrap(), (&link).callback(|e: KeyboardEvent| Msg::KeyboardEvent(e)));

        Model {
            link: link.clone(),
            job: None,
            event_listener: event_listener,
            board: board,
            movings: None,
            is_started: false,
            is_gameover: false,
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::info(&format!("{:?}", msg));
        match msg {
            Msg::Start => {
                self.is_started = true;
                self.is_gameover = false;
                self.board = ( 0..ROWS )
                .map(|_i| {
                    ( 0..COLS ).map(|_i| {
                        Color::Empty
                    } ).collect()
                }).collect();
                let handle =
                    IntervalService::spawn(Duration::from_millis(1), self.link.callback(|_| Msg::Generate ));
                self.job = Some(Box::new(handle));
            }
            Msg::Fall => {
                let (is_falling, board) = fall(self.board.clone());
                self.board = board;
                let handle =
                    IntervalService::spawn(
                        Duration::from_millis(if is_falling { 200 } else { 500 }),
                        self.link.callback(move |_| {if is_falling { Msg::Fall } else { Msg::Erase }}),
                    );
                self.job = Some(Box::new(handle));
            }
            Msg::Erase => {
                let (is_erased, board) = check_erase(self.board.clone());
                self.board = board;
                let handle =
                    IntervalService::spawn(
                        Duration::from_millis(if is_erased { 500 } else { 200 }),
                        self.link.callback(move |_| {if is_erased { Msg::Fall } else { Msg::Generate }}),
                    );
                self.job = Some(Box::new(handle));
            }
            Msg::Generate => {
                if self.board[1][2] != Color::Empty {
                    let handle =
                        IntervalService::spawn(Duration::from_millis(SPAWN_FALL_DERATION_MILLISEC),
                            self.link.callback(|_| Msg::GameOver),
                    );
                    self.job = Some(Box::new(handle));
                }else{
                    let movings = (
                        MovingRus{ x: 2, y: 0, color: random_color(),},
                        MovingRus{ x: 2, y: 1, color: random_color(),},
                    );
                    self.board[movings.0.y][movings.0.x] = movings.0.color;
                    self.board[movings.1.y][movings.1.x] = movings.1.color;

                    self.movings = movings.into();
                    let handle =
                        IntervalService::spawn(Duration::from_millis(SPAWN_FALL_DERATION_MILLISEC),
                            self.link.callback(|_| Msg::MovingsFall),
                    );
                    self.job = Some(Box::new(handle));
                }

            }
            Msg::MovingsFall => {
                let mut movings = self.movings.clone().unwrap();
                if movings.0.y == COLS as usize || movings.1.y == COLS as usize || self.board[movings.1.y + 1][movings.1.x] != Color::Empty{
                    self.board[movings.0.y][movings.0.x] = movings.0.color;
                    self.board[movings.1.y][movings.1.x] = movings.1.color;
                    self.movings = None;
                    let handle =
                        IntervalService::spawn(Duration::from_millis(SPAWN_FALL_DERATION_MILLISEC),
                            self.link.callback(|_| Msg::Fall),
                        );
                    self.job = Some(Box::new(handle));
                } else {
                    self.board[movings.0.y][movings.0.x] = Color::Empty;
                    self.board[movings.1.y][movings.1.x] = Color::Empty;

                    movings.0.y += 1;
                    movings.1.y += 1;

                    self.board[movings.0.y][movings.0.x] = movings.0.color;
                    self.board[movings.1.y][movings.1.x] = movings.1.color;
                    self.movings = movings.into();

                    let handle =
                        IntervalService::spawn(Duration::from_millis(SPAWN_FALL_DERATION_MILLISEC),
                            self.link.callback(|_| Msg::MovingsFall),
                    );
                    self.job = Some(Box::new(handle));
                }

            }
            Msg::KeyboardEvent(e) => {
                ConsoleService::info(&e.key());
                if self.movings.is_none() {
                    return true;
                }
                // ConsoleService::info(&format!("{:?}", &self.movings));
                let mut movings = self.movings.clone().unwrap();
                match e.key().as_str() {
                    "d" => {
                        ConsoleService::info("KeyboardEvent Right");
                        if movings.0.x >= (COLS - 1) as usize {
                            return true;
                        }
                        if self.board[movings.0.y][movings.0.x + 1] == Color::Empty && self.board[movings.1.y][movings.1.x + 1] == Color::Empty{
                            self.board[movings.0.y][movings.0.x] = Color::Empty;
                            self.board[movings.1.y][movings.1.x] = Color::Empty;

                            movings.0.x += 1;
                            movings.1.x += 1;

                            self.board[movings.0.y][movings.0.x] = movings.0.color;
                            self.board[movings.1.y][movings.1.x] = movings.1.color;
                            self.movings = movings.into();
                        }
                    }
                    "a" => {
                        ConsoleService::info("KeyboardEvent Left");
                        ConsoleService::info(&format!("{:?}", movings.0.x));
                        if movings.0.x <= 0 {
                            return true
                        }
                        if self.board[movings.0.y][movings.0.x - 1] == Color::Empty && self.board[movings.1.y][movings.1.x - 1] == Color::Empty{
                            self.board[movings.0.y][movings.0.x] = Color::Empty;
                            self.board[movings.1.y][movings.1.x] = Color::Empty;

                            movings.0.x -= 1;
                            movings.1.x -= 1;

                            self.board[movings.0.y][movings.0.x] = movings.0.color;
                            self.board[movings.1.y][movings.1.x] = movings.1.color;
                            self.movings = movings.into();
                        }
                    }
                    "s" => {
                        ConsoleService::info("KeyboardEvent Down");
                        let handle =
                            IntervalService::spawn(Duration::from_millis(0),
                                self.link.callback(|_| Msg::MovingsFall),
                            );
                        self.job = Some(Box::new(handle));
                     }
                    // "w" => { ConsoleService::info("KeyboardEvent Rotate") }
                    _ => ()
                }
            }
            Msg::GameOver => {
                self.is_gameover = true;
                self.job = None
            }
            // _ => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let view_cell = |cell:&rus::Color| {
            html! { <Rus color={cell.clone()} /> }
        };
        let view_row = |row:&Vec<rus::Color>| {
            html! { <div class="stage_row">{ for row.iter().map(view_cell) }</div> }
        };
        let start_button = || {
            if !self.is_started {
                html! { <button
                    class="start-button"
                    onclick=self.link.callback(|_| Msg::Start)>{ "すた〜と" }</button>
                }
            } else {
                html! {}
            }
        };
        let game_over = || {
            ConsoleService::info(&format!("{:?}", self.is_gameover));
            if self.is_gameover {
                html! {
                    <>
                        <div class="batankyu">{"ばたんきゅ〜"}</div>
                        <button
                        class="start-button"
                        onclick=self.link.callback(|_| Msg::Start)>{ "りすた〜と" }</button>
                    </>
                }
            } else {
                html! {}
            }
        };

        html! {
            <>
                <div class="stage">
                    { for self.board.iter().map(view_row) }
                    { start_button() }
                    { game_over() }
                </div>
                <div class="description">
                    <p>{"a: 左に動く"}</p>
                    <p>{"d: 右に動く"}</p>
                    <p>{"s: 下に落ちる"}</p>
                </div>
            </>
        }
    }
}

fn fall(_board: Vec<Vec<Color>>) -> (bool, Vec<Vec<Color>>){
    // ConsoleService::info(&format!("{:?}", _board));
    let mut board = _board.clone();
    let mut is_falling = false;
    board.reverse();

    for x in 0..(COLS as usize) {
        for y in 0..(ROWS as usize){
            if board[y][x] == Color::Empty {
                continue;
            }
            let cell:Color = board[y][x];
            if y > 0 as usize && board[y - 1][x] == Color::Empty {
                board[y][x] = Color::Empty;
                board[y - 1][x] = cell;
                is_falling = true;
            }
        }
    }

    board.reverse();

    return (is_falling, board)
}

fn check_erase(_board: Vec<Vec<Color>>) -> (bool, Vec<Vec<Color>>){
    let mut board = _board.clone();
    let mut existing_rus_info_list: Vec<(usize, usize, Color)> = vec![];
    let mut is_erased = false;
    for y in 0..ROWS {
        for x in 0..COLS {
            let mut sequence_rus_info_list: Vec<(usize, usize, Color)> = vec![];
            check_sequential_rus(x as usize, y as usize, &mut sequence_rus_info_list, &mut board);
            if sequence_rus_info_list.len() >= MIN_ERASE_CHAIN {
                is_erased = true;
            }else{
                existing_rus_info_list.append(&mut sequence_rus_info_list);

            }
        }
    }

    for info in existing_rus_info_list {
        board[info.1][info.0] = info.2;
    }

    return (is_erased, board);
}

fn check_sequential_rus(x: usize, y: usize, sequence_rus_info_list: &mut Vec<(usize, usize, Color)>, board: &mut Vec<Vec<Color>>) {
    if board[y][x] == Color::Empty {
        return;
    }
    let rus = board[y][x].clone();
    sequence_rus_info_list.push((
        x as usize,
        y as usize,
        rus,
    ));
    board[y][x] = Color::Empty;

    let direction:Vec<(i64, i64)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for d in direction {
        let dx = x as i64 + d.0;
        let dy = y as i64 + d.1;
        if dx < 0 || dx >= COLS.into() || dy < 0 || dy >= ROWS.into() {
            continue;
        }

        let dx = dx as usize;
        let dy = dy as usize;
        if board[dy][dx] != rus {
            continue;
        }
        check_sequential_rus(dx, dy, sequence_rus_info_list, board)
    }
}

