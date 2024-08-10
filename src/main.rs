use promkit::preset::listbox::Listbox;

mod action;
mod git;
mod hexo;

/// 入口選項
const ACTION: [&str; 5] = [
    "建立新文章",
    "開啟預覽",
    "commit",
    "發布到測試站",
    "發布到正式站",
];

fn main() {
    let repo = git::get_repo();
    welcome().unwrap();
}

/// 入口，和使用者互動詢問他們這次要什麼功能
fn welcome() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Listbox::new(ACTION)
        .title("要做什麼呢？")
        .listbox_lines(5)
        .prompt()?;
    match p.run()?.as_str() {
        "建立新文章" => action::create(),
        _ => Ok(println!("不支援的選項")),
    }
    .expect("入口選項互動發生錯誤");

    Ok(())
}
