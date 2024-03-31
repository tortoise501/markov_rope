use std::{cell::RefCell, collections::HashMap, default, fs::File, io::Read, path::Path, rc::Rc};

use rand::Rng;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



type RefRc<T> = Rc<RefCell<T>>;

fn ref_rc_new<T>(obj:T) -> RefRc<T>{
    Rc::new(RefCell::new(obj))
}

pub struct MarkovChain<'a>{
    text:TrainingText,
    map: Option<HashMap<&'a str,HashMap<&'a str,u32>>>,
}
impl<'a> MarkovChain<'a> {
    pub fn new(text:TrainingText) -> MarkovChain<'a>{
        MarkovChain{
            text,
            map:None,
        }
    }
    pub fn default() -> MarkovChain<'a> {
        MarkovChain::new(TrainingText::Default)
    }
    fn get_text(&mut self) -> String{
        match &self.text {
            TrainingText::String(text) => {text.clone()}
            TrainingText::Path(path) => {match File::open(path){
                Ok(mut contents) => {
                    let mut answer = String::new();
                    contents.read_to_string(&mut answer).unwrap();
                    answer
                    // todo!()
                },
                Err(err) => panic!("problem with opening the file"),
            }
        }
            TrainingText::Default => String::from("test text for testing purposes, change in to be normal text about something general so that the answer is not about something specific like computer games and fantasy worlds"),
        }
    }
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
                            let insert = map.insert(word.clone(), hsmap);
                        },
                        None => {
                            let mut hsmap = words_to_count.clone();
                            hsmap.insert(next_word.clone(), 1);
                            map.insert(word.clone(), hsmap);
                        },
                    }
                }
                None => {
                    let mut hsmap = HashMap::new();
                    hsmap.insert(next_word.clone(), 1);
                    map.insert(word.clone(), hsmap);
                },
                
            }
        }
        self.map = Some(map);
    }
    
    fn get_next_after(&self,word:&str) -> String {
        if let Some (map) = &self.map {
            println!("choosing for: {}",word);
            match map.get(&word) {
                Some(hs) => {
                    println!("{} is in map",word);
                    let mut kv_pairs:Vec<(&str,&u32)> = Vec::new();
                    let keys = hs.keys();
                    for key in keys{
                        kv_pairs.push((key,hs.get(key).unwrap()))
                    }
                    println!("chose pool: {:?}",kv_pairs);
                    let mut total_val:u32 = 0;
                    for kv in &kv_pairs {
                        total_val += *kv.1;
                    }
                    let mut rng = rand::thread_rng();
                    let rng = rng.gen_range(1..=total_val);

                    println!("where rng: {}, and tv: {}",rng,total_val);
                    let mut passed :u32= 0;
                    for i in &kv_pairs{
                        passed+=i.1;
                        if rng as i32 - (passed as i32)< *i.1 as i32{
                            println!("chosen: {}",i.0);
                            return i.0.to_string()
                        }
                    }
                    return kv_pairs[0].0.to_string()
                },
                None => {
                    println!("{} isn't in map",word);
                    let keys = map.keys();
                    let keys:Vec<&&str> = keys.collect();
                    let mut rng = rand::thread_rng();
                    println!("{}",keys.len());
                    let rng = rng.gen_range(0..keys.len());
                    return self.get_next_after(keys[rng].clone());
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
            last_word.push(' ');
            answer.push_str(last_word.as_str());
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
        let mut mc = MarkovChain::new(TrainingText::String("a b a b a v a b a b a v".to_string()));
        println!("{}",mc.get_text());
        let text = mc.get_text();
        mc.generate_map(&text);
        println!("{:?}",mc.map);

        println!("{}",mc.generate_text(5));

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}