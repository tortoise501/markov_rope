use std::{collections::HashMap, path::Path};

use rand::Rng;

pub struct MarkovChain<'a>{
    map: Option<HashMap<&'a str,HashMap<&'a str,u32>>>,
}
impl<'a> MarkovChain<'a> {
    pub fn new() -> MarkovChain<'a>{
        MarkovChain{
            map:None,
        }
    }
    pub fn from(text: &'a String) -> MarkovChain<'a> {
        let mut chain = MarkovChain{
            map:None,
        };
        chain.generate_map(text);
        chain
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
    pub fn generate_text(self,word_count:u32)-> Result<String,String>{
        if self.map.iter().count() < 1{
            return Err("map is not created".to_string());
        }
        let mut answer = String::new();
        let mut last_word = String::from("");
        for _ in 1..word_count{
            let word = self.get_next_after(last_word.as_str());
            last_word = word;
            // last_word.push(' ');
            answer.push_str(last_word.as_str());
            answer.push(' ');
        } 
        Ok(answer)
    }
}
pub enum TrainingText {
    String(String),
    Path(Box<Path>),
    Default,
}

#[cfg(test)]
mod test {
    use crate::MarkovChain;

    #[test]
    fn test_text_generation(){
        let sample_text = "I an a string used to generate text for unit test.".to_string();
        let chain = MarkovChain::from(&sample_text);
        let word_count = 10;
        let generated_text = chain.generate_text(word_count);
        assert!(generated_text.is_ok());
        let generated_text = generated_text.unwrap();
        let generated_text_word_count = generated_text.split(' ').count();
        assert_eq!(generated_text_word_count,word_count as usize);
    }
    fn test_for_no_map(){
        let chain = MarkovChain::new();
        let res = chain.generate_text(50);
        assert!(res.is_err())
    }
}