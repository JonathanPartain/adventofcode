import System.IO
import Data.List

-- take string, turn into tuple
--lineToTup :: String -> (String, String)
--lineToTup line = splitOn " " line
handleLine :: String -> [Int]
handleLine str = map read (words str)

cnt :: [Int] -> Int -> Int
cnt lst val = length ( filter (==val) lst)

main :: IO()
main = do
    let filePath = "../input.txt"
    fileData <- readFile filePath
    -- ["3   4","4   3","2   5","1   3","3   9","3   3"] lines fileData
    -- sum of a map of abs x-y where [[x],[y]] comes from transpose. sorted. Get the filedata, handle the line, transpose, sort and do the first
    print $ sum $ map (\(a:b:_) -> abs (a-b)) $ transpose $ map sort $ transpose $ map handleLine $ lines fileData

    -- https://errorgorn.github.io/2024/12/01/Haskell.html
    -- would not have solved part two without this reference
    -- get the co
    -- find the count of x in y
    -- transpose $ map sort $ transpose $ map handleLine $ lines fileData
    let ll:rl:_ = transpose $ map handleLine $ lines fileData
    -- https://hackage.haskell.org/package/base-4.20.0.1/docs/Prelude.html#v:zipWith
    print $ sum $ zipWith(*) ll $ map (cnt rl) ll
