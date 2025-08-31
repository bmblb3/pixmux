use color_eyre::Result;

mod app;
mod tab;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tab::Tab;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.current_tab(), Tab::Data);
    }

    #[test]
    fn test_tab_switching() {
        let mut app = App::new();
        assert_eq!(app.current_tab(), Tab::Data);

        app.next_tab();
        assert_eq!(app.current_tab(), Tab::Image);

        app.next_tab();
        assert_eq!(app.current_tab(), Tab::Data);
    }

    #[test]
    fn test_tab_titles() {
        let mut app = App::new();
        assert_eq!(app.current_tab().title(), "Data");

        app.next_tab();
        assert_eq!(app.current_tab().title(), "Image");
    }

    #[test]
    fn test_set_tab() {
        let mut app = App::new();
        app.set_tab(Tab::Image);
        assert_eq!(app.current_tab().title(), "Image");
    }
}
