let hello name = Printf.printf "Hi %s!\n" name
let name = Sys.getenv "USER";;

hello name
