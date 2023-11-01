pub mod fmt {
    use std::fmt::Display;

    use anstyle::{AnsiColor, Reset, Style};
    use anyhow::Error;

    pub fn plain(err: &Error) -> impl Display {
        format!("{}", Plain(err))
    }

    struct Plain<'a>(&'a Error);

    impl Plain<'_> {
        const ERROR: &'static str = "error";
        const CAUSE: &'static str = "cause";

        const BOLD: Style = Style::new().bold();
        const STYLE: Style = AnsiColor::Red.on_default();
    }

    impl Display for Plain<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                concat!("{BOLD}{STYLE}{ERROR}{RESET}", "{BOLD}:{RESET} ", "{error}"),
                BOLD = Self::BOLD.render(),
                STYLE = Self::STYLE.render(),
                RESET = Reset.render(),
                ERROR = Self::ERROR,
                error = self.0
            )?;
            for cause in self.0.chain().skip(1) {
                write!(
                    f,
                    concat!("\n", "{STYLE}{CAUSE}{RESET}", "{BOLD}:{RESET} ", "{cause}"),
                    BOLD = Self::BOLD.render(),
                    STYLE = Self::STYLE.render(),
                    RESET = Reset.render(),
                    CAUSE = Self::CAUSE,
                    cause = cause,
                )?;
            }
            Ok(())
        }
    }
}
