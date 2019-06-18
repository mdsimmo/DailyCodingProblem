/**

Suppose we represent our file system by a string in the following manner:

The string "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext" represents:

dir
    subdir1
    subdir2
        file.ext

The directory dir contains an empty sub-directory subdir1 and a sub-directory subdir2 containing a file file.ext.

The string "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext" represents:

dir
    subdir1
        file1.ext
        subsubdir1
    subdir2
        subsubdir2
            file2.ext

The directory dir contains two sub-directories subdir1 and subdir2. subdir1 contains a file file1.ext and an empty second-level sub-directory subsubdir1. subdir2 contains a second-level sub-directory subsubdir2 containing a file file2.ext.

We are interested in finding the longest (number of characters) absolute path to a file within our file system. For example, in the second example above, the longest absolute path is "dir/subdir2/subsubdir2/file2.ext", and its length is 32 (not including the double quotes).

Given a string representing the file system in the above format, return the length of the longest absolute path to a file in the abstracted file system. If there is no file in the system, return 0.

Note:

The name of a file contains at least a period and an extension.

The name of a directory or sub-directory will not contain a period.


Solution:
    Pretty obvious really. We return longest path instead of longest path length.

Time to complete: 40 min

*/

pub fn longest_file(input: &str) -> String {

    let mut longest_str = "".to_string();
    let mut pre_depth = 0;
    let mut path = vec![];
    for s in input.split("\n") {
        // for some reason, CLion can't infer type of s
        // so I explicitly declare it here
        let p:&str = s;

        // Adjust the path to be current
        let depth = p.chars().take_while(|&c| c == '\t').count();
        for _ in depth..=pre_depth {
            path.pop();
        }
        pre_depth = depth;
        // remove leading tabs and push section to path
        path.push(p.split_at(depth).1);

        // test if this is the longest file
        let is_file = p.contains('.');
        if is_file {
            let file_path = path.join("/");
            if file_path.len() > longest_str.len() {
                longest_str = file_path;
            }
        }
    }

    return longest_str;
}

#[cfg(test)]
mod test {
    use longest_file;

    #[test]
    fn test_simple_file_struct() {
        let input = "one\n\ttwo\n\t\tfile.txt";
        let longest = "one/two/file.txt";

        assert_eq!(longest_file(input), longest);
    }

    #[test]
    fn test_multiple_layers() {
        let input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
        let longest = "dir/subdir2/subsubdir2/file2.ext";

        assert_eq!(longest_file(input), longest);
    }

    #[test]
    fn test_multiple_layers_first_file_is_best() {
        let input = "dir\n\tsubdir1\n\t\tsubsubdir1\n\t\t\tfile1_long.ext\n\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
        let longest = "dir/subdir1/subsubdir1/file1_long.ext";

        assert_eq!(longest_file(input), longest);
    }

    #[test]
    fn test_no_files() {
        let input = "dir\n\tsubdir";
        let longest = "";

        assert_eq!(longest_file(input), longest);
    }

    #[test]
    fn test_empty() {
        let input = "";
        let longest = "";

        assert_eq!(longest_file(input), longest);
    }

}