mod History;
mod commit;
mod create_history;
mod get_current_state;
mod redo;
mod undo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
