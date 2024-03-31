use std::{cell::RefCell, collections::HashMap, default, fs::File, io::Read, path::Path, rc::Rc};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



type RefRc<T> = Rc<RefCell<T>>;

fn ref_rc_new<T>(obj:T) -> RefRc<T>{
    Rc::new(RefCell::new(obj))
}

pub struct MarkovChain{
    text:TrainingText,
    map: Option<HashMap<String,HashMap<String,u32>>>,
}
impl MarkovChain {
    pub fn new(text:TrainingText) -> MarkovChain{
        MarkovChain{
            text,
            map:None,
        }
    }
    pub fn default() -> MarkovChain {
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
    pub fn generate_map(&mut self){
        if self.map.is_none(){
            self.map = Some(HashMap::new());
        }
        let mut map = self.map.take().unwrap();
        let text = self.get_text();
        let words: Vec<&str> = text.split(' ').collect();
        for (i,word) in words[..words.len()-1].into_iter().enumerate() {
            let next_word = words[i + 1];
            match map.get(*word){

                Some(words_to_count)=>{
                    match words_to_count.get(next_word) {
                        Some(count) => {
                            let mut hsmap = words_to_count.clone();
                            hsmap.insert(next_word.to_string(), count + 1);//KYS
                            map.insert(word.to_string(), hsmap);
                        },
                        None => {
                            let mut hsmap = words_to_count.clone();
                            hsmap.insert(next_word.to_string(), 1);
                            map.insert(word.to_string(), hsmap);
                        },
                    }
                }
                None => {
                    let mut hsmap = HashMap::new();
                    hsmap.insert(next_word.to_string(), 1);
                    map.insert(word.to_string(), hsmap);
                },
                
            }
        }
        self.map = Some(map);
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
        mc.generate_map();
        println!("{:?}",mc.map);


        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}