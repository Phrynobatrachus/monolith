//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod passing {
    use monolith::html;
    use monolith::opts::Options;

    #[test]
    fn isolated() {
        let options = Options {
            isolate: true,
            ..Default::default()
        };
        let csp_content = html::compose_csp(&options);

        assert_eq!(
            csp_content,
            "default-src 'unsafe-eval' 'unsafe-inline' data:;"
        );
    }

    #[test]
    fn no_css() {
        let options = Options {
            no_css: true,
            ..Default::default()
        };
        let csp_content = html::compose_csp(&options);

        assert_eq!(csp_content, "style-src 'none';");
    }

    #[test]
    fn no_fonts() {
        let options = Options {
            no_fonts: true,
            ..Default::default()
        };
        let csp_content = html::compose_csp(&options);

        assert_eq!(csp_content, "font-src 'none';");
    }

    #[test]
    fn no_frames() {
        let options = Options {
            no_frames: true,
            ..Default::default()
        };
        let csp_content = html::compose_csp(&options);

        assert_eq!(csp_content, "frame-src 'none'; child-src 'none';");
    }

    #[test]
    fn no_js() {
        let options = Options {
            no_js: true,
            ..Default::default()
        };
        let csp_content = html::compose_csp(&options);

        assert_eq!(csp_content, "script-src 'none';");
    }

    #[test]
    fn no_images() {
        let options = Options {
            no_images: true,
            ..Default::default()
        };
        let csp_content = html::compose_csp(&options);

        assert_eq!(csp_content, "img-src data:;");
    }

    #[test]
    fn all() {
        let options = Options {
            isolate: true,
            no_css: true,
            no_fonts: true,
            no_frames: true,
            no_js: true,
            no_images: true,
            ..Default::default()
        };
        let csp_content = html::compose_csp(&options);

        assert_eq!(csp_content, "default-src 'unsafe-eval' 'unsafe-inline' data:; style-src 'none'; font-src 'none'; frame-src 'none'; child-src 'none'; script-src 'none'; img-src data:;");
    }
}
