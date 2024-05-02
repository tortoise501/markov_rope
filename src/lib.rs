use std::{collections::HashMap, path::Path};

use rand::Rng;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



// type RefRc<T> = Rc<RefCell<T>>;

// fn ref_rc_new<T>(obj:T) -> RefRc<T>{
//     Rc::new(RefCell::new(obj))
// }

pub struct MarkovChain<'a>{
    map: Option<HashMap<&'a str,HashMap<&'a str,u32>>>,
}
impl<'a> MarkovChain<'a> {
    pub fn new() -> MarkovChain<'a>{
        MarkovChain{
            map:None,
        }
    }
    // fn get_text(&mut self) -> String{
    //     match &self.text {
    //         TrainingText::String(text) => {text.clone()}
    //         TrainingText::Path(path) => {match File::open(path){
    //             Ok(mut contents) => {
    //                 let mut answer = String::new();
    //                 contents.read_to_string(&mut answer).unwrap();
    //                 answer
    //                 // todo!()
    //             },
    //             Err(_err) => panic!("problem with opening the file"),
    //         }
    //     }
    //         TrainingText::Default => String::from("test text for testing purposes, change in to be normal text about something general so that the answer is not about something specific like computer games and fantasy worlds"),
    //     }
    // }
    pub fn generate_map(&mut self,text: &'a String){
        if self.map.is_none(){
            self.map = Some(HashMap::new());
        }
        let mut map = self.map.take().unwrap();
        // let text = self.get_text();
        let words: Vec<&str> = text.split(' ').collect();
        for (i,word) in words[..words.len()-1].into_iter().enumerate() {
            let next_word = words[i + 1];
            match map.get(*word){

                Some(words_to_count)=>{
                    match words_to_count.get(next_word) {
                        Some(count) => {
                            let mut hsmap = words_to_count.clone();
                            hsmap.insert(next_word, count + 1);//KYS
                            let _insert = map.insert(word, hsmap);
                        },
                        None => {
                            let mut hsmap = words_to_count.clone();
                            hsmap.insert(next_word, 1);
                            map.insert(word, hsmap);
                        },
                    }
                }
                None => {
                    let mut hsmap = HashMap::new();
                    hsmap.insert(next_word, 1);
                    map.insert(word, hsmap);
                },
                
            }
        }
        self.map = Some(map);
    }
    
    fn get_next_after(&self,word:&str) -> String {
        if let Some (map) = &self.map {
            //println!("choosing for: {}",word);
            match map.get(&word) {
                Some(hs) => {
                    //println!("{} is in map",word);
                    let mut kv_pairs:Vec<(&str,&u32)> = Vec::new();
                    let keys = hs.keys();
                    for key in keys{
                        kv_pairs.push((key,hs.get(key).unwrap()))
                    }
                    //println!("chose pool: {:?}",kv_pairs);
                    let mut total_val:u32 = 0;
                    for kv in &kv_pairs {
                        total_val += *kv.1;
                    }
                    let mut rng = rand::thread_rng();
                    let rng = rng.gen_range(1..=total_val);

                    //println!("where rng: {}, and tv: {}",rng,total_val);
                    let mut passed :u32= 0;
                    for i in &kv_pairs{
                        passed+=i.1;
                        if rng as i32 - (passed as i32)< *i.1 as i32{
                            //println!("chosen: {}",i.0);
                            return i.0.to_string()
                        }
                    }
                    return kv_pairs[0].0.to_string()
                },
                None => {
                    //println!("{} isn't in map",word);
                    let keys = map.keys();
                    let keys:Vec<&&str> = keys.collect();
                    let mut rng = rand::thread_rng();
                    //println!("{}",keys.len());
                    let rng = rng.gen_range(0..keys.len());
                    return self.get_next_after(keys[rng]);
                    // FromIterator::from_iter(this)
                },
            }
        }
        todo!()
    }
    pub fn generate_text(self,word_count:u32)-> String{
        let mut answer = String::new();
        let mut last_word = String::from("");
        for _ in 0..word_count{
            let word = self.get_next_after(last_word.as_str());
            last_word = word;
            // last_word.push(' ');
            answer.push_str(last_word.as_str());
            answer.push(' ');
        } 
        answer
    }
}
pub enum TrainingText {
    String(String),
    Path(Box<Path>),
    Default,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut mc = MarkovChain::new();
        // //println!("{}",mc.get_text());
        let text = "It wasn’t always so clear, but the Rust programming language is fundamentally about empowerment: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before. Take, for example, “systems-level” work that deals with low-level details of memory management, data representation, and concurrency. Traditionally, this realm of programming is seen as arcane, accessible only to a select few who have devoted the necessary years learning to avoid its infamous pitfalls. And even those who practice it do so with caution, lest their code be open to exploits, crashes, or corruption. Rust breaks down these barriers by eliminating the old pitfalls and providing a friendly, polished set of tools to help you along the way. Programmers who need to “dip down” into lower-level control can do so with Rust, without taking on the customary risk of crashes or security holes, and without having to learn the fine points of a fickle toolchain. Better yet, the language is designed to guide you naturally towards reliable code that is efficient in terms of speed and memory usage. Programmers who are already working with low-level code can use Rust to raise their ambitions. For example, introducing parallelism in Rust is a relatively low-risk operation: the compiler will catch the classical mistakes for you. And you can tackle more aggressive optimizations in your code with the confidence that you won’t accidentally introduce crashes or vulnerabilities. But Rust isn’t limited to low-level systems programming. It’s expressive and ergonomic enough to make CLI apps, web servers, and many other kinds of code quite pleasant to write — you’ll find simple examples of both later in the book. Working with Rust allows you to build skills that transfer from one domain to another; you can learn Rust by writing a web app, then apply those same skills to target your Raspberry Pi. This book fully embraces the potential of Rust to empower its users. It’s a friendly and approachable text intended to help you level up not just your knowledge of Rust, but also your reach and confidence as a programmer in general. So dive in, get ready to learn—and welcome to the Rust community!".to_string();
        mc.generate_map(&text);
        //println!("{:?}",mc.map);

        //println!("{}",mc.generate_text(50));

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}