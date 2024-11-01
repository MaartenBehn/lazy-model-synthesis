mod node;

use crate::go_back_in_time::history::node::{HistoryNode, HistoryNodePacker, SummaryIndex};
use crate::go_back_in_time::node::HistoryIndex;
use crate::general_data_structure::identifier::PackedIdentifierT;
use crate::general_data_structure::value::ValueDataT;

#[derive(Default, Clone)]
pub struct History<NodeStorage: Clone> {
    pub packer: HistoryNodePacker,
    
    pub nodes: Vec<HistoryNode>,
    pub summaries: Vec<NodeStorage>
}

impl<NodeStorage: Clone> History<NodeStorage> {
    pub fn new(num_values: usize) -> Self {

        let packer = HistoryNodePacker::new(num_values);
        
        History {
            packer,
            nodes: vec![],
            summaries: vec![],
        }
    }

    pub fn add_summary(&mut self, node_storage: NodeStorage) {
        debug_assert!(self.summaries.len() < (1 << 31), "Summaries are full!");

        let history_node = self.packer.pack_summary(self.summaries.len() as SummaryIndex);
        self.nodes.push(history_node);
        self.summaries.push(node_storage);
    }

    pub fn add_change<I: PackedIdentifierT, VD: ValueDataT>(&mut self, packed_identifier: I, value_data: VD) -> HistoryIndex {
        let history_node = self.packer.pack_change(packed_identifier, value_data);
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

    pub fn get_change<I: PackedIdentifierT, VD: ValueDataT>(&self, index: usize) -> (I, VD) {
        self.packer.unpack_change(self.nodes[index])
    }
    
    pub fn last_index(&self) -> HistoryIndex {
        if self.nodes.is_empty() {
            return 0
        }
        
        (self.nodes.len() - 1) as HistoryIndex
    }
    
    pub fn last_summary_before_change(&self, mut index: usize) -> usize {
        let mut summary_index = 0;
        for i in (0..index).rev() {
            if self.nodes[i].is_summary() {
                summary_index = i;
                break
            }
        }
        
        summary_index
    }
    
    pub fn remove_all_after_with_last_summary_index(&mut self, index: usize, summary_index: usize) {
        self.nodes.truncate(index + 1);
        self.summaries.truncate(summary_index + 1);
    }

}

