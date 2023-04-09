use shell_gpt_rs::ask_chatgpt;

fn main() {
    let input = "show the list of files in the current directory, show details of files, show size in human readable way";
    println!("Input: {input}");
    let res = ask_chatgpt(input);
    println!("{:?}", res);
}
