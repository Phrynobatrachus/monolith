//  ██████╗  █████╗ ███████╗███████╗██╗███╗   ██╗ ██████╗
//  ██╔══██╗██╔══██╗██╔════╝██╔════╝██║████╗  ██║██╔════╝
//  ██████╔╝███████║███████╗███████╗██║██╔██╗ ██║██║  ███╗
//  ██╔═══╝ ██╔══██║╚════██║╚════██║██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║███████║███████║██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod passing {
    use reqwest::Url;
    use std::env;

    use monolith::opts::Options;
    use monolith::url;
    use monolith::utils;

    #[test]
    fn read_data_url() {
        let options = Options {
            silent: true,
            ..Default::default()
        };

        // If both source and target are data URLs,
        //  ensure the result contains target data URL
        let (data, final_url, media_type, charset) = utils::retrieve_asset(
            &Url::parse("data:text/html;base64,c291cmNl").unwrap(),
            &Url::parse("data:text/html;base64,dGFyZ2V0").unwrap(),
            &options,
            0,
        )
        .unwrap();
        assert_eq!(&media_type, "text/html");
        assert_eq!(&charset, "US-ASCII");
        assert_eq!(
            url::create_data_url(&media_type, &charset, &data, &final_url),
            Url::parse("data:text/html;base64,dGFyZ2V0").unwrap(),
        );
        assert_eq!(
            final_url,
            Url::parse("data:text/html;base64,dGFyZ2V0").unwrap(),
        );
    }

    #[test]
    fn read_local_file_with_file_url_parent() {
        let options = Options {
            silent: true,
            ..Default::default()
        };

        let file_url_protocol: &str = if cfg!(windows) { "file:///" } else { "file://" };

        // Inclusion of local assets from local sources should be allowed
        let cwd = env::current_dir().unwrap();
        let (data, final_url, media_type, charset) = utils::retrieve_asset(
            &Url::parse(&format!(
                "{file}{cwd}/tests/_data_/basic/local-file.html",
                file = file_url_protocol,
                cwd = cwd.to_str().unwrap()
            ))
            .unwrap(),
            &Url::parse(&format!(
                "{file}{cwd}/tests/_data_/basic/local-script.js",
                file = file_url_protocol,
                cwd = cwd.to_str().unwrap()
            ))
            .unwrap(),
            &options,
            0,
        )
        .unwrap();
        assert_eq!(&media_type, "application/javascript");
        assert_eq!(&charset, "");
        assert_eq!(url::create_data_url(&media_type, &charset, &data, &final_url), Url::parse("data:application/javascript;base64,ZG9jdW1lbnQuYm9keS5zdHlsZS5iYWNrZ3JvdW5kQ29sb3IgPSAiZ3JlZW4iOwpkb2N1bWVudC5ib2R5LnN0eWxlLmNvbG9yID0gInJlZCI7Cg==").unwrap());
        assert_eq!(
            final_url,
            Url::parse(&format!(
                "{file}{cwd}/tests/_data_/basic/local-script.js",
                file = file_url_protocol,
                cwd = cwd.to_str().unwrap()
            ))
            .unwrap()
        );
    }
}

//  ███████╗ █████╗ ██╗██╗     ██╗███╗   ██╗ ██████╗
//  ██╔════╝██╔══██╗██║██║     ██║████╗  ██║██╔════╝
//  █████╗  ███████║██║██║     ██║██╔██╗ ██║██║  ███╗
//  ██╔══╝  ██╔══██║██║██║     ██║██║╚██╗██║██║   ██║
//  ██║     ██║  ██║██║███████╗██║██║ ╚████║╚██████╔╝
//  ╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝

#[cfg(test)]
mod failing {
    use reqwest::Url;

    use monolith::opts::Options;
    use monolith::utils;

    #[test]
    fn read_local_file_with_data_url_parent() {
        let options = Options {
            silent: true,
            ..Default::default()
        };

        // Inclusion of local assets from data URL sources should not be allowed
        if let Ok((..)) = utils::retrieve_asset(
            &Url::parse("data:text/html;base64,SoUrCe").unwrap(),
            &Url::parse("file:///etc/passwd").unwrap(),
            &options,
            0,
        ) {
            panic!();
        }
    }

    #[test]
    fn read_local_file_with_https_parent() {
        let options = Options {
            silent: true,
            ..Default::default()
        };

        // Inclusion of local assets from remote sources should not be allowed
        if let Ok((..)) = utils::retrieve_asset(
            &Url::parse("https://kernel.org/").unwrap(),
            &Url::parse("file:///etc/passwd").unwrap(),
            &options,
            0,
        ) {
            panic!();
        }
    }
}
