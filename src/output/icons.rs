use ansi_term::Style;
use fs::File;
use info::filetype::FileExtensions;
use output::file_name::FileStyle;

pub trait FileIcon {
    fn icon_file(&self, file: &File) -> Option<char>;
}

pub enum Icons {
    Android,
    Apple,
    Audio,
    Compressed,
    Conf,
    Crypto,
    Css,
    Doc,
    Ebook,
    Font,
    Git,
    Hs,
    Image,
    Java,
    Json,
    Jsx,
    Md,
    Ppt,
    Py,
    R,
    Rb,
    Rubydoc,
    Shell,
    Styl,
    Tex,
    Video,
    Vim,
    Windows,
    Xls,
    Xml,
    Yml,
}

impl Icons {
    pub fn value(&self) -> char {
        match *self {
            Icons::Android    => '\u{e70e}',
            Icons::Apple      => '\u{f179}',
            Icons::Audio      => '\u{f001}',
            Icons::Compressed => '\u{f410}',
            Icons::Conf       => '\u{e615}',
            Icons::Crypto     => '\u{f83d}',
            Icons::Css        => '\u{e749}',
            Icons::Doc        => '\u{f1c2}',
            Icons::Ebook      => '\u{e28b}',
            Icons::Font       => '\u{f031}',            
            Icons::Git        => '\u{f1d3}',
            Icons::Hs         => '\u{e777}',
            Icons::Image      => '\u{f1c5}',            
            Icons::Java       => '\u{e204}',
            Icons::Json       => '\u{e60b}',
            Icons::Jsx        => '\u{e7ba}',
            Icons::Md         => '\u{f48a}',
            Icons::Ppt        => '\u{f1c4}',
            Icons::Py         => '\u{e606}',
            Icons::R          => '\u{f25d}',
            Icons::Rb         => '\u{e21e}',
            Icons::Rubydoc    => '\u{e73b}',
            Icons::Shell      => '\u{f489}',
            Icons::Styl       => '\u{e600}',
            Icons::Tex        => '\u{e600}',
            Icons::Video      => '\u{f03d}',
            Icons::Vim        => '\u{e62b}',
            Icons::Windows    => '\u{f17a}',
            Icons::Xls        => '\u{f1c3}',
            Icons::Xml        => '\u{e619}',
            Icons::Yml        => '\u{f481}',
        }
    }
}

pub fn painted_icon(file: &File, style: &FileStyle) -> String {
    let file_icon = icon(&file).to_string();
    let painted = style.exts
            .colour_file(&file)
            .map_or(file_icon.to_string(), |c| { 
                // Remove underline from icon
                if c.is_underline {
                    match c.foreground {
                        Some(color) => Style::from(color).paint(file_icon).to_string(),
                        None => Style::default().paint(file_icon).to_string(),
                    }
                } else {
                    c.paint(file_icon).to_string() 
                }
            });
    format!("{} ", painted)
}

fn icon(file: &File) -> char {
    let extensions = Box::new(FileExtensions);
    if file.is_directory() { 
        '\u{f115}' 
    }
    else if let Some(icon) = extensions.icon_file(file) { icon }
    else { 
        if let Some(ext) = file.ext.as_ref() {
            match ext.as_str() {
                "ai" => '\u{e7b4}',
                "audio" => '\u{f001}',
                "avro" => '\u{e60b}',
                "c" => '\u{e61e}',
                "clj" => '\u{e768}',
                "coffee" => '\u{f0f4}',
                "cpp" => '\u{e61d}',
                "d" => '\u{e7af}',
                "dart" => '\u{e798}',
                "db" => '\u{f1c0}',
                "diff" => '\u{f440}',
                "dropbox" => '\u{f16b}',
                "env" => '\u{f462}',
                "erl" => '\u{e7b1}',
                "font" => '\u{f031}',
                "gform" => '\u{f298}',
                "go" => '\u{e626}',
                "html" => '\u{f13b}',
                "image" => '\u{f1c5}',
                "iml" => '\u{e7b5}',
                "js" => '\u{e74e}',
                "less" => '\u{e758}',
                "log" => '\u{f18d}',
                "lua" => '\u{e620}',
                "mustache" => '\u{e60f}',
                "npmignore" => '\u{e71e}',
                "pdf" => '\u{f1c1}',
                "php" => '\u{e73d}',
                "pl" => '\u{e769}',
                "psd" => '\u{e7b8}',
                "rdb" => '\u{e76d}',
                "rs" => '\u{e7a8}',
                "rss" => '\u{f09e}',
                "sass" => '\u{e603}',
                "scala" => '\u{e737}',
                "sqlite3" => '\u{e7c4}',
                "ts" => '\u{e628}',
                "twig" => '\u{e61c}',
                "txt" => '\u{f15c}',
                "video" => '\u{f03d}',
                "windows" => '\u{f17a}',
                "xls" => '\u{f1c3}',
                _ => '\u{f15b}'
            }
        } else {
            '\u{f15b}'
        }
    }
}
