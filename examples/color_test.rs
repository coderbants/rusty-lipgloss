fn main() {
    let s = charming_lipgloss::new_style()
        .foreground("909090")
        .inline(true);
    println!("{:?}", s.render("key"));
}
