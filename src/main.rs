use promkit::preset::confirm::Confirm;
use promkit::preset::listbox::Listbox;
use promkit::preset::readline::Readline;

mod git;
mod hexo;

fn main() {
    let repo = git::get_repo();
    create().unwrap();
}

fn create() -> Result<(), Box<dyn std::error::Error>> {
    // 先選分類
    let mut p_category = Listbox::new(hexo::get_scaffolds())
        .title("請選擇要建立的新文章分類")
        .listbox_lines(10)
        .prompt()?;
    let category = p_category.run()?;
    println!("選擇了 {:?}", category);

    // 請求文章標題
    let mut p_title = Readline::default()
        .title("請輸入文章標題")
        .validator(
            |text| text.len() > 0,
            |text| format!("請輸入至少一個字元的文章標題: {}", text),
        )
        .prompt()?;
    let title = p_title.run()?;
    println!("文章標題 {:?}", title);

    // 最後確認
    let mut p_confirm = Confirm::new(format!(
        "即將建立分類 {} 標題為 {} 的文章，請確認是否正確",
        title, category
    ))
    .prompt()?;
    if p_confirm.run()?.to_lowercase().contains("y") {
        hexo::new(&title, &category);
    } else {
        println!("取消");
    }

    Ok(())
}
