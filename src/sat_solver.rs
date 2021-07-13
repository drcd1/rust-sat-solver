use crate::implication_graph::ImplicationGraph;

use crate::util::lit2idx;

pub struct SatSolver{
    vars: usize,
    clauses:  Vec<Clause3>,
    workspace:  Vec<Clause3>,
    assigns: Vec<i32>,
    var_to_clause: Vec<Vec<usize>>,
    graph: ImplicationGraph,
    sat: i32,
}

impl SatSolver{
    pub fn new(vn: usize) -> SatSolver{
        SatSolver{
            vars: vn,
            clauses: Vec::new(),
            workspace: Vec::new(),
            assigns: vec![0;vn],
            var_to_clause: vec![Vec::new();vn],
            graph: ImplicationGraph::new(vn),
            sat: 0
        }
        
    }
    pub fn add_clause(&mut self, v: Vec<i32>){
        println!("Vars: {}", self.vars);
        if v.len() < 1{
            self.sat = -1;
            return;
        }
        match v.len(){
            1 => self.add_clause_3([v[0],0,0]),
            2 => self.add_clause_3([v[0],v[1],0]),
            3 => self.add_clause_3([v[0],v[1],v[2]]),
            _ => self.add_general_clause(v),
        
        }

    }
    
        
    fn add_clause_3(&mut self, clause: [i32;3]){
        self.clauses.push(Clause3{
                            literals: clause,
                            sat: false
                            });
        for l in clause{         
            if l!=0 {   
                //println!("literal: {}, {}",l, self.var_to_clause.len());
                self.var_to_clause[lit2idx(l)].push(self.clauses.len()-1);
            }
        }
    }

    pub fn print(&self){
        for i in 0..self.clauses.len() {
            let &c = &self.clauses[i];
            println!("Clause {}: {} {} {}",i+1, c.literals[0],c.literals[1],c.literals[2])
        }
        for i in 0..self.var_to_clause.len() {
            print!("Var to clause: {}: ",i+1);
            for idx in &self.var_to_clause[i]{
                print!("{}, ", idx+1);
            }
            println!("");
        }
    }

    pub fn print_solution(&self){
        if self.sat == 0 {
            println!("UNSOLVED!");
        } else if self.sat == -1 {
            println!("UNSAT");
        }else {
            for i in 0..n_vars{
                println!("{}: {}", i+1, match self.assigns[i] {0=>{"Don't Care"} 1=>{"True"} _=>{"False"}});
            }
        }
    }

    pub fn check_solution(&self){
        println!("Check solution: {} {}",self.workspace.len(),self.clauses.len());
        for i in 0..n_clauses{
            print!("Clause {} ",i+1);

            /*
            match self.clauses[i].sat{
               true => {print!("sat :");}
                false => {print!("not sat :");}

            }
*/
            let mut sat = false;
            for j in 0..3{
                let l = self.clauses[i].literals[j];
                if l != 0 {
                    if l.signum() == self.assigns[lit2idx(l)]{
                        sat = true;
                        print!("is SAT because of {}", l*l.signum());
                        break;
                    }
                }
                
            }
            if !sat {
                print!("is UNSAT!");
                    
                println!("");
                println!("IS NOT A SOLUTION");
                break;
            }
            println!();
        }
    }

    fn increase_vars(&mut self){
        self.vars+=1;
        self.assigns.push(0);
        self.var_to_clause.push(Vec::new())
    }

    fn add_general_clause(&mut self,v: Vec<i32>){
        self.increase_vars();
        self.add_clause_3([v[0],v[1],self.vars as i32]);
        if v.len()%2 == 0{
            for i in 2..(v.len()){
                self.increase_vars();
                self.add_clause_3([-((self.vars-1)as i32),v[i],self.vars as i32]);
            }
            self.add_clause_3([-(self.vars as i32),v[v.len()-1],0]);
        } else {
            for i in 2..(v.len()-1){
                self.increase_vars();
                self.add_clause_3([-((self.vars-1) as i32),v[i],self.vars as i32]);
            }
            self.add_clause_3([-(self.vars as i32),v[v.len()-1],v[v.len()-2]]);
        }

    }


    fn solve_conflict(&mut self,clause: usize){
        let c = self.learn_clause(clause);
        //print!("Learned clause: ");
       // for i in &c {
       //     print!("{} ", *i);
        //}
       // println!();
        self.add_clause(c);
    }


    fn unit_propagate(&mut self, literal: i32, clause: usize){
        self.add_propagation_to_graph(literal, clause);
        self.set_literal(literal);        
    }

    fn decide_literal(&mut self, literal: i32){
        self.add_decision_to_graph(literal);
        self.set_literal(literal);
    }

    fn add_propagation_to_graph(&mut self,literal: i32, clause: usize){        
        let c = self.clauses[clause];
        let mut c2 = [0,0];
        let mut j = 0;
        for i in 0..3{
            if c.literals[i] != literal{
                c2[j] = c.literals[i].abs();
                j+=1;
            }
        }
        self.graph.add_child(literal,c2);
    }
    fn add_decision_to_graph(&mut self,literal: i32){
        self.graph.add_root(literal);
    }


    fn learn_clause(&mut self, clause: usize) -> Vec<i32> {
        let (roots, childs) = self.graph.get_conflict(&self.clauses[clause]);
        //TODO: check if copy
        //TODO: roots already has sign?
        let mut clause = roots.to_vec();
        /*
        for l in &mut clause{
            if self.assigns[(*l-1) as usize] == 1{
                *l = -*l;
            }
        }
        */
       // print!("Learned clause BEFORE: ");
       //for i in &clause {
       //     print!("{} ", *i);
       // }
      //  println!();
        /*
        for l in &mut clause{
            *l = -*l;
        }
        */
        for l in &mut clause{
            if self.assigns[(*l-1) as usize] == 1{
                *l = -*l;
            }
        }

        /*
        print!("Learned clause AFTER: ");
        for i in &clause {
            print!("{} ", *i);
        }
        println!();
        */
        self.unset_vars(roots);
        self.unset_vars(childs);

        return clause;
    }


    fn set_literal(&mut self, literal: i32){        
        let idx = lit2idx(literal);
        if(self.assigns[idx]!= 0){
            panic!("already assigned {}", literal);
        }
        //println!("Assigning: {} as  {}", idx+1,literal);
        self.assigns[idx] = literal.signum();
        for c_id in &self.var_to_clause[idx]{
            //TODO WHY THE * AND THE &
            let c = &mut self.workspace[*c_id];

            //we only want to look at clauses that are yet unsat
            if c.sat {continue };

            //check where is the literal
            //and put it in
            for i in 0..3{
                if c.literals[i] == literal {
                    c.literals[i] = 0;
                    //println!("Sat clause {}, at {}, with {}", c_id+1,i,literal);
                    c.sat = true;
                    break;
                } else if c.literals[i] == -literal {
                    c.literals[i] = 0;
                    break;
                }  
            }
        }
    }

    

    fn unset_vars(&mut self, vars: Vec<i32>){
        for l in &vars{
            self.unset_var(lit2idx(*l) as usize);
        }
    }

    fn unset_var(&mut self, idx: usize){
        //println!("unset_vars {}",idx+1);
        self.assigns[idx] = 0;
        //TODO: check if for loop changes anything
        let help = &self.var_to_clause[idx];
        for &c_id in help {
            //println!("Reset clause {}", c_id+1);            
            self.workspace[c_id] = self.clauses[c_id].clone();
            

            //clause check
            for i in 0..3{
                if self.workspace[c_id].literals[i] != 0 {
                    if self.workspace[c_id].literals[i].signum() == self.assigns[lit2idx(self.workspace[c_id].literals[i])]{
                        //println!("clause {} is sat by {}",c_id+1,self.workspace[c_id].literals[i]);
                        self.workspace[c_id].sat = true;
                        self.workspace[c_id].literals[i] = 0;
                    } else if self.workspace[c_id].literals[i].signum() == -self.assigns[lit2idx(self.workspace[c_id].literals[i])] {
                        self.workspace[c_id].literals[i] = 0;
                    }
                }
            }
        }
    }

    pub fn solve(&mut self){
        if self.clauses.len() == 0 {
            return;
        }

        //start from the end
        let mut current_clause : i32 = (self.clauses.len()-1) as i32;
        //TODO: check if deep copy
        self.workspace =  self.clauses.to_vec();

        //note: maybe start elsewhere        
        while self.sat == 0 {
            let c = &mut self.workspace[current_clause as usize];
            //println!("{}: {} {} {}",current_clause+1,c.literals[0],c.literals[1],c.literals[2]);

            if c.sat == true {
                current_clause-=1
            } else {
                //println!("Is not sat");
                //attributed variables
                let mut literal = 0;
                let mut attr = 3;
                for i in 0..3{
                    //TODO: do we need to check if c.literals is 0?
                    //if c.literals[i] is unassigned and we are not choosing a variable
                    //we'll chose a variable
                    if c.literals[i] != 0 /*&& self.assigns[lit2idx(c.literals[i])] == 0*/ && literal == 0
                    {
                        literal = c.literals[i];
                    } //else if the literal has been assigned, decrease teh counter of attributable lits
                    else if c.literals[i] == 0{
                        attr-=1;
                    }
                }
                //if there was a literal to be assigned
                if literal!=0 {
                    //and only one
                    if attr == 1 {
                        //note: I should do up as soon as possible
                        //i.e.: not here
                        self.unit_propagate(literal, current_clause as usize);
                    } else /* it's a decision ^^ */ {
                        self.decide_literal(literal);
                    }
                    
                    current_clause-=1;
                } else /*can't satisfy!*/{
                    //println!("can't satisfy");
                    //note: should detect conflict earlier
                    self.solve_conflict(current_clause as usize);
                    current_clause = (self.clauses.len()-1) as i32;
                    println!("{} clauses!",current_clause);
                    //TODO do I need cloned?
                    self.workspace.extend(self.clauses[self.workspace.len()..self.clauses.len()].iter().cloned());
                }
                
            }

            //TODO: make sure we always do this!
            if current_clause == -1 {
                self.sat = 1;
            }
        
        }

    }

}


#[derive(Clone,Copy)]
pub struct Clause3{
    pub literals: [i32;3],
    pub sat: bool
}

impl Default for Clause3{
    fn default() -> Clause3{
        Clause3{
            literals: [0,0,0],
            sat: false
        }
    }
}