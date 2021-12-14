pub mod submarine;
pub mod utils;
pub mod grid;
pub mod bingo;
pub mod ocean_vents;
pub mod lanternfish;
pub mod display_troubleshooting;
pub mod either;
pub mod heightmap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
