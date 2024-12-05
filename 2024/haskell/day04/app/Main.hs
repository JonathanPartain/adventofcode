module Main where

import qualified Data.Map as M
import Data.Maybe (catMaybes)
import Text.Regex.TDFA

createList :: [String] -> [String]
createList [] = []
createList (x : xs) = [x] ++ createList xs

extractXmas :: String -> Int
extractXmas input =
  let pattern = "(XMAS)|(SAMX)"
   in input =~ pattern :: Int

main :: IO ()
main = do
  let file_path = "small.txt"
  file_data <- readFile file_path
  print $ lines file_data
  let l = createList $ lines file_data

  let ll = map (map (: [])) l
  print ll

--  print $ transpose ll
--
--  let rows = [extractXmas x | subl <- ll, x <- subl]
--  let cols = [extractXmas x | subl <- transpose ll, x <- subl]
--  print rows
--  -- let v = map extractXmas $ transpose l
--  print cols
--  --
--  let diag2 = diagonals $ transpose ll
--  let test = [extractXmas x | subl <- diagonals ll, x <- subl]
--  print test
--
--  let test2 = [extractXmas x | subl <- diagonals diag2, x <- subl]
--  print test2
--
-- print h

-- print v

-- let horiz = map $ extract_xmas l
-- let vert = map $ extract_xmas $ transpose l
-- print horiz
-- print vert
