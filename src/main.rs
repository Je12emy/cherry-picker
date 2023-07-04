use cherry_picker::Picker;

fn main() {
    let options = vec!["Option A", "Option B"];
    let mut picker = Picker::new(&options);
    picker.select();
    println!("{:?}", picker.selected_options_indexes);
}
