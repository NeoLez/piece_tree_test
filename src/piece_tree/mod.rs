use crate::fixed_length_string::FixedSizeString;

struct SkipList {
    first_node : Node,
    length_nodes : u32,
    length_graphemes : u32,
    length_lines : u32,
}

impl SkipList {
    fn new(node : Node) -> Self {
        SkipList {
            length_graphemes : node.length_graphemes,
            length_lines : node.newline_range,
            first_node : node,
            length_nodes : 1,
        }
    }
}

struct Node {
    buffer_index : u16,
    start : u32,
    length_graphemes : u32,
    newline_ind_start : u32,
    newline_range : u32,
    next_nodes : Vec<(Option<Node>, u32, u32)>
}

struct PieceTree {
    buffers : Vec<FixedSizeString>,
    cursor : (u32, u32),
    nodes : SkipList,
}

impl PieceTree {
    fn new(original_text : String) -> Self {
        let buffer = FixedSizeString::from_string(original_text);
        let newline_range = buffer.line_starts().len() as u32;
        let length_graphemes = buffer.len() as u32;
        PieceTree{
            buffers : vec![buffer],
            cursor : (0,0),
            nodes : {
                let list: SkipList = SkipList::new(
                    Node {
                        buffer_index : 0,
                        start : 0,
                        length_graphemes,
                        newline_ind_start : 0,
                        newline_range,
                        next_nodes : vec![]
                    });
                list
            }
        }
    }
    /*fn create_node(&mut self, s : String) -> (Node, Option<Node>){
        /*
        NO ADD BUFFERS -> Create a new buffer with the size of the string added and return that
        IF THERE ARE ADD BUFFERS:
            IF IT FITS INSIDE THE BUFFER -> return the node added, allocate string to existing buffer
            IF IT DOESNT FIT -> fill the existing add buffer to its full capacity and:
                IF THE SIZE OF THE REMAINDER TEXT IS SMALLER OR EQUAL THAN THE SIZE OF THE PREVIOUS
                BUFFER * 2 -> create a new buffer with the size of the previous * 2 and paste there
                IF IT ISN'T: create a buffer with the size of the text to paste 
        */
        if self.buffers.len() > 1 {
            let mut a = self.buffers.last_mut().unwrap();
            let buf_index1 = a.
            if a.append(&s) >
            (,None)
        }else{
            self.buffers.push(FixedSizeString::from_string(s));
            let b = self.buffers[1];
            (Node{
                buffer_index : 0,
                start : 0,
                length_graphemes : self.buffers[1].len(),
                newline_ind_start : 0,
                newline_range : self.buffers[1].line_starts().len() as u32,
                next_nodes : vec![],
            }, None)
        }
    }
    fn add(&mut self, s : String, ind : u32) -> Result<(),()> {
        if ind > self.nodes.length_graphemes {
            return Result::Err(())
        }

    }*/
}