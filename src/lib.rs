use std::collections::HashMap;

use rand::Rng;
/// A struct used to generate text 
pub struct MarkovChain<'a>{
    map: Option<HashMap<&'a str,HashMap<&'a str,u32>>>,
}

const DEFAULT_TEXT: &str ="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut fringilla maximus nulla quis gravida. Suspendisse potenti. Curabitur sem leo, tempus vitae rutrum in, dignissim nec sem. Etiam at varius dolor, eget efficitur mauris. Donec id diam ullamcorper, varius ante at, malesuada ante. Quisque egestas ac libero quis elementum. Maecenas vitae ex nisl. Donec urna eros, convallis ut purus congue, efficitur dapibus tortor. Pellentesque euismod laoreet dolor et posuere. Mauris viverra ante et mattis mollis. Donec pellentesque et ligula vitae ultricies. Nullam auctor dignissim diam id gravida. Maecenas vehicula pulvinar urna. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Nam non viverra sem. Nulla ex augue, bibendum quis purus a, eleifend viverra nisl. In tempus ante quis nunc scelerisque mollis. Donec facilisis elit nec augue convallis viverra. Nam vel pharetra massa. Phasellus at consequat elit, et euismod ante. Suspendisse vestibulum ante a lorem tempus rutrum. Nunc euismod mi quis tellus consectetur placerat. Nullam vitae est et turpis aliquam convallis. Praesent egestas molestie urna, ac malesuada risus blandit sed. Nam quam ligula, iaculis non justo nec, pharetra pharetra diam. Donec at tempus ipsum. Quisque sit amet tempor massa. Vivamus rutrum enim mi, quis suscipit sem laoreet quis. Vivamus magna enim, posuere id pharetra quis, elementum eget ante. Curabitur tincidunt pulvinar magna convallis ullamcorper. Integer quis mattis risus. Duis vel quam vitae urna eleifend rhoncus in at metus. Aliquam pulvinar magna imperdiet neque mattis, ac dapibus sem dignissim. Nullam sit amet interdum magna, vel venenatis est. Vestibulum tristique ultrices ipsum eu venenatis. Phasellus congue malesuada sagittis. ";

impl<'a> MarkovChain<'a> {
    /// Creates new empty chain. 
    /// You need to generate map to generate new text if tou created a chain using `new()`
    /// # Example
    /// ```
    /// let mut chain = MarkovChain::new();
    /// chain.generate_map("text used to generate your map");
    /// println!("{}",chain.generate_text(10).unwrap());
    /// ```
    pub fn new() -> MarkovChain<'a>{
        MarkovChain{
            map:None,
        }
    }
    /// Generates a markov chain with map generated using string literal
    /// 
    /// # Example
    /// ```
    /// let chain = MarkovChain::from_string("your string here");
    /// println!("{}",chain.generate_text(10).unwrap());
    /// ```
    pub fn from_string(source_text: &'a str) -> MarkovChain<'a> {
        let mut chain = MarkovChain{
            map:None,
        };
        chain.generate_map(source_text);
        chain
    }
    ///
    /// Generates a markov chain with map generated using default text
    /// 
    /// # Example
    /// ```
    /// let chain = MarkovChain::default();
    /// println!("{}",chain.generate_text(10).unwrap());
    /// ```
    pub fn default() -> MarkovChain<'a> {
        let mut chain = MarkovChain{
            map:None,
        };
        chain.generate_map(DEFAULT_TEXT);
        chain
    }
    ///
    /// Generates a map from given text. Map is later used to generate text
    /// 
    pub fn generate_map(&mut self,text: &'a str){
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
    /// Generates text using previously generated map. Returns error if map is not generated
    /// 
    /// # Example
    /// ```
    /// let chain = MarkovChain::from_string("your string here");
    /// match chain.generate_text(10){
    ///     Ok(generated_text) => println!("{}",generated_text),
    ///     Err(e) => panic!("failed to generate text{}",e),
    /// };
    /// ```
    pub fn generate_text(&self,word_count:u32)-> Result<String,String>{
        if self.map.iter().count() < 1{
            return Err("map is not created".to_string());
        }
        let mut answer = String::new();
        let mut last_word = String::from("");
        for _ in 1..word_count{
            let word = self.get_next_after(last_word.as_str());
            last_word = word;
            answer.push_str(last_word.as_str());
            answer.push(' ');
        } 
        Ok(answer)
    }
}

#[cfg(test)]
mod test {
    use super::MarkovChain;

    #[test]
    fn test_text_generation(){
        let sample_text = "I an a string used to generate text for unit test.".to_string();
        let chain = MarkovChain::from_string(&sample_text);
        let word_count = 10;
        let generated_text = chain.generate_text(word_count);
        assert!(generated_text.is_ok());
        let generated_text = generated_text.unwrap();
        let generated_text_word_count = generated_text.split(' ').count();
        assert_eq!(generated_text_word_count,word_count as usize);
    }
    #[test]
    fn test_for_no_map(){
        let chain = MarkovChain::new();
        let res = chain.generate_text(50);
        assert!(res.is_err())
    }
}