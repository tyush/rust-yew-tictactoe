use yew::prelude::*;


fn main() {
    yew::start_app::<Board>();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileState {
    Empty,
    X,
    O,
}

impl TileState {
    fn to_ch(self) -> char {
        match self {
            TileState::Empty => ' ',
            TileState::X => 'X',
            TileState::O => 'O',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlayerTurn {
    X,
    O,
}

impl PlayerTurn {
    fn other(self) -> PlayerTurn {
        match self {
            PlayerTurn::O => PlayerTurn::X,
            PlayerTurn::X => PlayerTurn::O,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BoardMsg {
    TileClicked(usize, usize),
    Reset,
}

enum EndType {
    XWin,
    OWin,
    Draw
}

struct Board {
    link: ComponentLink<Self>,
    winner: Option<EndType>,
    current_turn: PlayerTurn,
    ///  map: y ->
    ///    -------
    ///  x | x o o
    ///  | | x x o
    ///  V | o x
    board: Vec<Vec<TileState>>,
}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            winner: None,
            current_turn: PlayerTurn::X,
            board: vec![
                vec![TileState::Empty, TileState::Empty, TileState::Empty],
                vec![TileState::Empty, TileState::Empty, TileState::Empty],
                vec![TileState::Empty, TileState::Empty, TileState::Empty],
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BoardMsg::TileClicked(x, y) => {
                if self.board[x][y] != TileState::Empty || self.winner.is_some() {
                    self.reset();
                    true
                } else {
                    self.board[x][y] = {
                        match self.current_turn {
                            PlayerTurn::X => TileState::X,
                            PlayerTurn::O => TileState::O,
                        }
                    };
                    self.winner = self.check_win();
                    self.current_turn = self.current_turn.other();
                    true
                }
            }
            BoardMsg::Reset => {
                self.reset();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <table>
                <tr>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(0, 0))}>{self.board[0][0].to_ch()}</button></td>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(0, 1))}>{self.board[0][1].to_ch()}</button></td>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(0, 2))}>{self.board[0][2].to_ch()}</button></td>
                </tr>
                <tr>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(1, 0))}>{self.board[1][0].to_ch()}</button></td>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(1, 1))}>{self.board[1][1].to_ch()}</button></td>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(1, 2))}>{self.board[1][2].to_ch()}</button></td>
                </tr>
                <tr>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(2, 0))}>{self.board[2][0].to_ch()}</button></td>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(2, 1))}>{self.board[2][1].to_ch()}</button></td>
                    <td><button onclick={self.link.callback(|_| BoardMsg::TileClicked(2, 2))}>{self.board[2][2].to_ch()}</button></td>
                </tr>
                <tr>
                    <td>
                        <button onclick={self.link.callback(|_| BoardMsg::Reset)}>{"reset"}</button>
                    </td>
                    <td>{
                        if self.winner.is_some() {
                            match &self.winner.as_ref().unwrap() {
                                EndType::XWin => "X wins the game!",
                                EndType::OWin => "O wins the game!",
                                EndType::Draw => "Draw between X/O"
                            }
                        } else {
                            " "
                        }
                    }</td>
                </tr>
            </table>
        }
    }
}

impl Board {
    fn check_win(&self) -> Option<EndType> {
        let p = {match self.current_turn {
            PlayerTurn::O => TileState::O,
            PlayerTurn::X => TileState::X
        }};

        let end_type = {
            match self.current_turn {
                PlayerTurn::X => EndType::XWin,
                PlayerTurn::O => EndType::OWin
            }
        };

        for i in 0..=2 {
            if self.board[0][i] == p && self.board[1][i] == p && self.board[2][i] == p {
                return Some(end_type)
            }
            if self.board[i][0] == p && self.board[i][1] == p && self.board[i][2] == p {
                return Some(end_type)
            }
        };

        if self.board[0][0] == p && self.board[1][1] == p && self.board[2][2] == p {
            return Some(end_type)
        }
        if self.board[2][0] == p && self.board[1][1] == p && self.board[0][2] == p {
            return Some(end_type)
        };

        let mut no_more_turns = true;

        for row in &self.board {
            for tile in row {
                if tile == &TileState::Empty {
                    no_more_turns = false;
                }
            }
        }

        if no_more_turns {
            return Some(EndType::Draw)
        }

        None
    }

    fn reset(&mut self) {
        self.board = vec![
            vec![TileState::Empty, TileState::Empty, TileState::Empty],
            vec![TileState::Empty, TileState::Empty, TileState::Empty],
            vec![TileState::Empty, TileState::Empty, TileState::Empty],
        ];
        self.winner = None;
        self.current_turn = PlayerTurn::X;
    }
}