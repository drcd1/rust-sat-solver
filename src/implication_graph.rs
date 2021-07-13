
use std::collections::HashSet;
use crate::util::lit2idx;
use crate::sat_solver::Clause3;

pub struct ImplicationGraph{
    nodes: Vec<Node>,
    idx2node: Vec<usize>,
    free: Vec<usize>
}

impl ImplicationGraph {
    pub fn new(vsize: usize) -> ImplicationGraph{
        ImplicationGraph{
            nodes: Vec::new(),
            idx2node: vec![0;vsize],
            free: Vec::new(),
        }
    }

    fn extend(&mut self, idx: usize){
        if self.idx2node.len() > idx {
            return;
        }
        self.idx2node.extend(vec![0;idx-self.idx2node.len()+1]);
    }
    
    pub fn add_root(&mut self, literal: i32){
        //println!("add_root! {}", literal);
        self.extend(lit2idx(literal));
        let idx:usize;
        if self.free.len() == 0{
            self.nodes.push(Node{
                parents: [-1,-1],
                children: HashSet::new()
            });
            
            idx = self.nodes.len();
        } else {
            let a = self.free.pop();
            idx = a.unwrap();
            self.nodes[idx-1] =  Node {
                parents:[-1,-1],
                children: HashSet::new()
            }
        }

        self.idx2node[lit2idx(literal)] = idx;

    }

    fn rm_node(&mut self, literal: i32){
        
        let a = self.idx2node[lit2idx(literal)];

        
        //remove ref to children
        let node = &self.nodes[a-1];
        let mut p1 = 0;
        let mut p2 = 0;
        if node.parents[0]!=0 {
            p1 = self.idx2node[lit2idx(node.parents[0])];
        }
        if node.parents[1]!=0 {
            p2 = self.idx2node[lit2idx(node.parents[1])];
        }
        
        if p1>0 {
            let n = &mut self.nodes[p1-1];
            n.children.remove(&literal);
        }

        if p2>0 {
            let n = &mut self.nodes[p2-1];
            n.children.remove(&literal);
        }



        self.free.push(a);        
        //TODO: not needed, but explicit
        self.idx2node[lit2idx(literal)] = 0;


    }

    pub fn add_child(&mut self, literal: i32, clause: [i32;2]){
        //println!("add_child! {}", literal);
        //check
        /*
        if self.idx2node[lit2idx(clause[0])] == 0 || self.idx2node[lit2idx(clause[1])] == 0 {
            panic!("Oooops! literal {} has bad parents: {} {}", literal,clause[0], clause[1]);
        }*/
        
        self.extend(lit2idx(literal));
        let idx:usize;
        if self.free.len() == 0{
            self.nodes.push(Node{
                parents: clause,
                children: HashSet::new()
            });
            
            idx = self.nodes.len();
        } else {
            let a = self.free.pop();
            idx = a.unwrap();
            self.nodes[idx-1] =  Node {
                parents:clause,
                children: HashSet::new()
            }
        }

        for i in clause{
            if i!=0 {
                self.nodes[self.idx2node[lit2idx(i)]-1].children.insert(literal.abs());
            }
        }

        self.idx2node[lit2idx(literal)] = idx;
        //println!("idx2node yields: {}",self.idx2node[lit2idx(literal)]);
    }
    
    fn get_node(&self, literal: i32) -> &Node{
        return &self.nodes[self.idx2node[lit2idx(literal)]-1];
    }

    fn add_node_conflict(&mut self, roots:&mut  HashSet<i32>, children: &mut HashSet<i32>, literal: i32){
        if literal == 0 {
            return;
        }
        let p1;
        let p2;
        //print!("Add_node_conflict: {}", literal);
        {
        let help = self.get_node(literal);
        p1 = help.parents[0];
        p2 = help.parents[1];
        //print!(" - parents are {} {}\n", p1,p2);
        }

        if p1 == -1{
            roots.insert(literal.abs());
            self.insert_children(children,literal.abs());
        } else {
            if p1==0 &&p2==0 {
                children.insert(literal.abs());
                self.insert_children(children,literal.abs());
            }

            self.add_node_conflict(roots,children, p1);
            self.add_node_conflict(roots,children, p2);
            //todo: should I insert in child? no because I do it when I get to the root!
        }
    }
    fn insert_children(&self, children: &mut HashSet<i32>, literal: i32){
        //print!("Literal: {}. ",literal);
        //println!("Insert children: {} with children {:?}", literal,self.nodes[self.idx2node[lit2idx(literal)]-1].children);
        for &i in &self.nodes[self.idx2node[lit2idx(literal)]-1].children {
            children.insert(i.abs());
            self.insert_children(children,i.abs());   
        }
    }

    pub fn get_conflict(&mut self, clause: &Clause3) -> (Vec<i32>,Vec<i32>) {
        
        let mut roots = HashSet::new();
        let mut children = HashSet::new();
        for l in clause.literals {
            self.add_node_conflict(&mut roots, &mut children, l.abs());
        }
        let rs = roots.into_iter().collect();
        let cs = children.into_iter().collect();

        for &l in &rs{
                
            //am i deleteing before dereferencing? how does rust deal?
            //note to  self: solved;
            self.rm_node(l);
        }
        for &l in &cs {
            self.rm_node(l);
        }

        return (rs,cs);

    }
    /*
    fn add_node(&self, nodes: &mut HashSet<i32>,node: i32){
        nodes.insert(node);
        let n = self.get_node(node);
        if n.parents[0] == -1 {
            return;
        }

        if n.parents[0] !=0 {

        }
    }
    */
}

struct Node{
    parents: [i32;2],
    children:  HashSet<i32>
}
