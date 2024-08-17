mod node;

use crate::history::node::{HistoryNode, HistoryNodePacker};
use crate::identifier::PackedIdentifierT;
use crate::node::HistoryIndex;
use crate::value::ValueNr;

#[derive(Default, Clone)]
pub struct History<NodeStorage> {
    pub packer: HistoryNodePacker,

    
    pub nodes: Vec<HistoryNode>,
    pub summaries: Vec<NodeStorage>
}

impl<NodeStorage> History<NodeStorage> {
    pub fn new(node_storage: NodeStorage, num_values: usize) -> Self {

        let packer = HistoryNodePacker::new(num_values);
        let initial_summary = packer.pack_summary(0);
        
        History {
            packer,
            nodes: vec![initial_summary],
            summaries: vec![node_storage],
        }
    }

    pub fn add_summary(&mut self) {
        debug_assert!(self.summaries.len() >= (1 << 31), "Summaries are full!");
        
        todo!()
    }

    pub fn add_change<I: PackedIdentifierT>(&mut self, packed_identifier: I, value_nr: ValueNr) -> HistoryIndex {
        let history_node = self.packer.pack_change(packed_identifier, value_nr);
        let index = self.nodes.len() as HistoryIndex;
        self.nodes.push(history_node);

        index
    }
    
    pub fn is_change(&self, index: usize) -> bool {
        self.nodes[index].is_change()
    }
    
    pub fn get_summary(&self, index: usize) -> &NodeStorage {
        let summary_index = self.packer.unpack_summary(self.nodes[index]);
        &self.summaries[summary_index as usize]
    }

    pub fn get_change<I: PackedIdentifierT>(&self, index: HistoryIndex) -> (I, ValueNr) {
        self.packer.unpack_change(self.nodes[index as usize])
    }
}

