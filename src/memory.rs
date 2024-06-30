use super::*;

impl Machine {
    /// Free heap-allocated segment
    pub fn free(&mut self, ptr: Pointer) -> Result<(), Error> {
        let segment = match ptr {
            Pointer::Stack(_) => &mut self.stack.stack,
            Pointer::Heap(_) => &mut self.heap,
            _ => return Err(Error::InvalidPointer),
        };

        let ptr = ptr.as_usize();
        if ptr >= segment.len() {
            return Err(Error::SegmentationFault);
        }

        if let Word::Int(len) = segment[ptr] {
            let len = len as usize;
            let end = ptr + len;
            let start = ptr + 1;

            // Checking if the slice pointers are valid
            if end > segment.len() || start > segment.len() || start >= end {
                return Err(Error::SegmentationFault);
            }

            // Freeing the segment by setting all elements to Word::Free
            for i in start..end {
                segment[i] = Word::Free;
            }

            segment[ptr] = Word::Free;
        } 
        else {
            return Err(Error::TypeMismatch);
        }

        Ok(())
    }


    // Allocates Word's on the Heap
    pub fn malloc(&mut self, len: usize) -> Result<Pointer, Error> {
        let mut start_index = None;
        let mut segment_length = 0;

        for (index, word) in self.heap.iter().enumerate() {
            match word {
                Word::Free => {
                    if segment_length == 0 {
                        start_index = Some(index);
                    }
                    segment_length += 1;

                    // +1 to fit the length
                    if segment_length >= len + 1 {
                        break;
                    }
                }
                _ => {
                    start_index = None;
                    segment_length = 0;
                }
            }
        }

        // Returns Pointer to suitable segment
        if let Some(start) = start_index {
            self.heap[start] = Word::Int(len as i64);
            return Ok(Pointer::Heap(start + 1)); 
        }

        // Expands heap if no suitable segments already
        let start_index = self.heap.len();
        self.heap.push(Word::Int(len as i64));
        for _ in 0..len {
            self.heap.push(Word::Int(0));
            self.hp += 1;
        }

        Ok(Pointer::Heap(start_index + 1)) 
    }

    /// Sets Element 
    pub fn setelem(&mut self, elem: Pointer, value: Word) -> Result<(), Error> {
        let segment = match elem {
            Pointer::Heap(_) => &mut self.heap,
            Pointer::Stack(_) => &mut self.stack.stack,
            _ => return Err(Error::InvalidPointer),
        };

        let elem_ptr = elem.as_usize();

        // Check that pointer is within bounds
        if elem_ptr >= segment.len() || segment[elem_ptr] == Word::Free {
            return Err(Error::SegmentationFault);
        }
        
        segment[elem_ptr] = value;

        Ok(())
    }

    /// Reads String from Memory
    pub fn read_arr(&self, ptr: Pointer) -> Result<Vec<Word>, Error> {
        let segment = match ptr {
            Pointer::Heap(_) => &self.heap,
            Pointer::Stack(_) => &self.stack.stack,
            _ => return Err(Error::InvalidPointer),
        };

        let ptr = ptr.as_usize();
        let len_ptr = ptr - 1;

        if ptr >= segment.len() {
            return Err(Error::SegmentationFault);
        }
        
        if len_ptr >= segment.len() {
            return Err(Error::SegmentationFault);
        }
        
        if let Word::Int(len) = segment[len_ptr] {
            let len = len as usize;

            let start = ptr;
            let end = start + len;
            let arr_slice = &segment[start..end];
            let arr = arr_slice.to_vec();
            
            Ok(arr)
        } 
        else {
            return Err(Error::InvalidPointer);
        }
    }
}
