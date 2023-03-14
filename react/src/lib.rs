use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct ComputeCell<'a, T> {
    dependencies: Vec<CellId>,
    dependents: Vec<ComputeCellId>,
    callbacks: Vec<CallbackId>,
    compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
    value: T
}

impl <'a, T> ComputeCell<'a, T> {
    pub fn new<F: Fn(&[T]) -> T + 'a>(initial: T, dependencies: &[CellId], compute_func: F) -> Self {
        let mut d = Vec::new();
        d.extend_from_slice(dependencies);

        Self {
            dependencies: d,
            dependents: Vec::new(),
            compute_func: Box::new(compute_func),
            callbacks: Vec::new(),
            value: initial
        }
    }
}

pub struct InputCell<T> {
    dependents: Vec<ComputeCellId>,
    value: T
}

impl <T> InputCell<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: initial,
            dependents: Vec::new()
        }
    }
}

pub struct IDSequence(usize);

impl IDSequence {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> usize {
        let value = self.0;
        self.0 += 1;
        value
    }
}

pub struct Reactor<'a, T> {
    input_sequence: IDSequence,
    compute_sequence: IDSequence,
    callback_sequence: IDSequence,
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, T>>,
    callbacks: Vec<Box<dyn FnMut(T) + 'a>>
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_sequence: IDSequence::new(),
            compute_sequence: IDSequence::new(),
            callback_sequence: IDSequence::new(),
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
            callbacks: Vec::new()
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let cell = InputCell::new(initial);
        self.input_cells.push(cell);
        let id = InputCellId(self.input_sequence.next());
        
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        for dependency in dependencies {
            match dependency {
                CellId::Input(id) => {
                    if id.0 >= self.input_cells.len() {
                        return Err(CellId::Input(*id));
                    };
                },
                CellId::Compute(id) => {
                   if id.0 >= self.compute_cells.len() {
                       return Err(CellId::Compute(*id));
                   };
                }
            };
        }

        let id = ComputeCellId(self.compute_sequence.next());
        self.hook_dependencies(id, dependencies);
        let value = compute_func(&self.get_many(dependencies));
        let cell = ComputeCell::new(value, dependencies, compute_func);
        self.compute_cells.push(cell);

        Ok(id)
    }

    fn hook_dependencies(&mut self, cell_id: ComputeCellId, dependencies: &[CellId]) {
        for dependency in dependencies {
            match dependency {
                CellId::Input(InputCellId(id)) => {
                    self.input_cells[*id].dependents.push(cell_id);
                },
                CellId::Compute(ComputeCellId(id)) => {
                    self.compute_cells[*id].dependents.push(cell_id);
                }
            };
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(InputCellId(i)) => self.input_cells.get(i).map(|cell| cell.value),
            CellId::Compute(ComputeCellId(i)) => self.compute_cells.get(i).map(|cell| cell.value)
        }
    }

    pub fn get_many(&self, ids: &[CellId]) -> Vec<T> {
        ids
            .into_iter()
            .filter_map(|&id| self.value(id))
            .collect()
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        id.0 < self.input_cells.len() && {
            self.input_cells[id.0].value = new_value;
            let mut changes = HashMap::new();

            for i in 0..self.input_cells[id.0].dependents.len() {
                self.recompute(self.input_cells[id.0].dependents[i], &mut changes);
            }

            for (ComputeCellId(cell_id), old_value) in changes {
                let value = self.compute_cells[cell_id].value;

                if value != old_value {
                    for i in 0..self.compute_cells[cell_id].callbacks.len() {
                        let callback_id = self.compute_cells[cell_id].callbacks[i].0;
                        self.callbacks[callback_id](value);
                    }
                }
            }

            true
        }
    }

    fn recompute(&mut self, id: ComputeCellId, changes: &mut HashMap<ComputeCellId, T>) {
        let old_value = self.compute_cells[id.0].value;
        let value = (self.compute_cells[id.0].compute_func)(&self.get_many(&self.compute_cells[id.0].dependencies));
        
        if value != old_value {
            self.compute_cells[id.0].value = value;
            changes.entry(id).or_insert(old_value);

            for i in 0..self.compute_cells[id.0].dependents.len() {
                self.recompute(self.compute_cells[id.0].dependents[i], changes);
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        (id.0 < self.compute_cells.len())
            .then(|| {
                let callback_id = CallbackId(self.callback_sequence.next());
                self.callbacks.push(Box::new(callback));
                self.compute_cells[id.0].callbacks.push(callback_id);

                callback_id
            })
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if cell.0 >= self.compute_cells.len() {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        if callback.0 >= self.callbacks.len() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        let initial_len = self.compute_cells[cell.0].callbacks.len();

        self.compute_cells[cell.0]
            .callbacks
            .retain(|&id| id.0 != callback.0);

        if initial_len == self.compute_cells[cell.0].callbacks.len() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        
        Ok(())
    }
}
