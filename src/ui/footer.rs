use pixmux::Tab;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::App;
use crate::app::events::Mode;

pub struct Footer;

impl Footer {
    pub fn render(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
        let (modetext, modecolor, modemsg, modemsgcolor) = match (app.mode, &app.current_tab) {
            (Mode::Navigation, Tab::Data) => (
                "NAV",
                Color::Yellow,
                vec!["q: Quit", "<Tab>: IMAGE tab", ": Navigate sheet rows"],
                Color::LightYellow,
            ),
            (Mode::Navigation, Tab::Image) => (
                "NAV",
                Color::Yellow,
                vec![
                    "q: Quit",
                    "<Tab>: DATA tab",
                    ": Navigate panes",
                    "l: LAY mode",
                    "i: IMG mode",
                ],
                Color::LightYellow,
            ),
            (Mode::Layout, _) => (
                "LAY",
                Color::Blue,
                vec![
                    "<Esc>: NAV mode",
                    "v: Split vertically",
                    "h: Split horizontally",
                    "x: Remove pane",
                    ": Resize pane",
                ],
                Color::Cyan,
            ),
            (Mode::Image, _) => (
                "LAY",
                Color::Green,
                vec![
                    "<Esc>: IMG mode",
                    "n: Next image",
                    ": Pan image",
                    "󱓉: Zoom",
                ],
                Color::LightGreen,
            ),
        };

        let mode_bar = Paragraph::new(Line::from(
            [
                vec![
                    Span::styled("█", Style::default().fg(modecolor)),
                    Span::styled(
                        modetext,
                        Style::default().bg(modecolor).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("█", Style::default().fg(modecolor)),
                ],
                modemsg
                    .iter()
                    .map(|msg| {
                        vec![
                            Span::styled(" ", Style::default()),
                            Span::styled(
                                " ".to_string() + *msg + " ",
                                Style::default().bg(modemsgcolor).fg(Color::DarkGray),
                            ),
                        ]
                    })
                    .collect::<Vec<Vec<Span>>>()
                    .concat(),
            ]
            .concat(),
        ));
        frame.render_widget(mode_bar, area);
    }
}
