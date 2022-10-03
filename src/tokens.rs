#[macro_export]
macro_rules! tokens {

    [$tokens_name:ident = {
        $($token_name:ident $([$($token_arg_type:ty),+])?),*
    }]

    => {
        #[allow(non_snake_case)]
        mod $tokens_name {
            #[derive(Debug)]
            pub enum Type {
                $($token_name $((Option<$($token_arg_type),+>))?),*
            }
            pub type Position         = $crate::tokens::Position;
            pub type Token            = $crate::tokens::Token<Type>;
            pub type List             = $crate::tokens::List<Type>;
            pub type ListSnapshot<'l> = $crate::tokens::ListSnapshot<'l, Type>;
        }
    };

}


pub struct Position {
    location : String,
    line     : u64,
    column   : u64
}
pub struct Token<T> {
    token : T,
    range : (Position, Position)
}
pub struct List<T> {
    tokens : Vec<Token<T>>,
    index  : usize
}
impl<T> List<T> {
    pub fn new() -> List<T> {
        return List {
            tokens : Vec::new(),
            index  : 0
        };
    }
    pub fn reset(&mut self) {
        self.set_index(0);
    }
    pub fn next(&mut self) -> Option<&Token<T>> {
        self.set_index(self.index + 1);
        return self.get();
    }
    pub fn get(&self) -> Option<&Token<T>> {
        return self.tokens.get(self.index);
    }
    pub fn set_index(&mut self, index : usize) {
        self.index = index;
    }
    pub fn snapshot(&mut self) -> ListSnapshot<'_, T> {
        return ListSnapshot::new(self, self.index);
    }
}
pub struct ListSnapshot<'l, T> {
    list  : &'l mut List<T>,
    index : usize
}
impl<'l, T> ListSnapshot<'l, T> {
    pub fn new(list : &'l mut List<T>, index : usize) -> ListSnapshot<'l, T> {
        return ListSnapshot {
            list  : list,
            index : index
        };
    }
    pub fn restore(&mut self) {
        self.list.set_index(self.index);
    }
}