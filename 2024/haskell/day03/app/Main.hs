import Data.List (isPrefixOf, transpose)
import Text.Parsec
import Text.Parsec.String (Parser)
import Text.Regex.TDFA
import Text.Regex.TDFA.Text ()

extractMuls :: String -> [[String]] -- [(Int, Int)]
extractMuls input =
  let pattern = "mul\\([0-9]{1,3},[0-9]{1,3}\\)"
   in input =~ pattern :: [[String]]

extractAll :: String -> [[String]] -- [(Int, Int)]
extractAll input =
  let pattern = "mul\\([0-9]{1,3},[0-9]{1,3}\\)|don't\\(\\)|do\\(\\)"
   in input =~ pattern :: [[String]]

mulParser :: Parser (Int, Int)
mulParser = do
  _ <- string "mul("
  x <- read <$> many1 digit
  _ <- char ','
  y <- read <$> many1 digit
  _ <- char ')'
  return (x, y)

parseMul :: [String] -> Either ParseError (Int, Int)
parseMul input = parse mulParser "" $ head input

processMuls :: [Either ParseError (Int, Int)] -> [Int]
processMuls parsed =
  [x * y | Right (x, y) <- parsed]

sumList :: [Int] -> Int
sumList [] = 0
sumList (x : xs) = x + sumList xs

-- TODO: Look through this and learn
processSecondList :: [String] -> [String]
processSecondList = snd . foldl processItem (False, [])
  where
    processItem (ignore, acc) item
      | "don't()" `isPrefixOf` item = (True, acc)
      | "do()" `isPrefixOf` item = (False, acc)
      | ignore = (ignore, acc)
      | "mul(" `isPrefixOf` item = (ignore, acc ++ [item])
      | otherwise = (ignore, acc)

main :: IO ()
main =
  do
    let filePath = "input.txt"
    -- let filePath = "small.txt"
    let second = "input.txt"
    fileData <- readFile filePath
    secondData <- readFile second
    let res = map parseMul $ extractMuls fileData
    print $ sumList $ processMuls res
    let solve_two = extractAll secondData

    -- let flatList = transpose second
    let secondRes = processSecondList $ head $ transpose solve_two
    print $ sumList $ processMuls $ [parseMul [x] | x <- secondRes]

-- print $ flatList
-- print $ map extractNums $ extractMuls fileData
--    print $ map extractNums $ lines $ fileData
