--main = do
--    let file_path = "small.txt"
--    file_data <- readFile file_path 
--    putStrLn file_data
--

-- return true if element a is in list
testelem :: (Eq a) => a -> [a] -> Bool
testelem _ [] = False
testelem n (x:xs) = n == x || (testelem n xs)

-- remove all duplicates from a given list
testnub :: (Eq a) => [a] -> [a]
testnub [] = []
testnub (x:xs)
    | testelem x xs = testnub xs -- if x in xs, run testnub with xs
    | otherwise = x : testnub xs -- x not in xs, prepend x to the recursive result

tisAsc :: [Int] -> Bool
tisAsc [] = True -- no element is ascending?
tisAsc [_] = True -- one element is always ascending
tisAsc (x:y:xs) = (x < y && (abs (x-y) < 4 )) && tisAsc (y:xs)

tisDesc :: [Int] -> Bool
tisDesc [] = True -- no element is ascending?
tisDesc [_] = True -- one element is always ascending
tisDesc (x:y:xs) = (x > y && (abs (x-y) < 4)) && tisDesc (y:xs)

-- DID NOT UNDERSTAND THIS
-- List of Edges, starting node, end node, pathExists
hasPath :: [(Int, Int)] -> Int -> Int -> Bool
hasPath [] a b = a == b -- empty list, see if they are the same?
hasPath xs x y
    | x == y = True
    | otherwise =
        let xs' = [ (n,m) |(n,m) <- xs, n /= x] in -- all tuples in xs where n!=x
        or [hasPath xs' m y | (n,m) <- xs, n == x]

countTrue :: [Bool] -> Int
countTrue = length . filter id
    
createList :: String -> [Int]
createList str = map read (words str)

sumTwo :: Int -> Int -> Int
sumTwo x y = x + y

-- chatgpt assist
-- input [1,2,3]
-- Output: [[2,3], [1,3], [1,2]]
removeOne :: [a] -> [[a]]
removeOne [] = []
-- concat body with map where you concat Head and removeOne body
removeOne (x:xs) = xs : map (x:) (removeOne xs)

-- still chatgpt
checkPermutes :: [Int] -> Bool
-- Create permutes with removeOne, assign it to perm
-- check if it is Ascending or Descending, as well as the diff min/max. 
-- If any is true, its okay
checkPermutes xs = any (\perm -> tisAsc perm || tisDesc perm) (removeOne xs)

main :: IO()
main = 
    do
    let filePath = "input.txt"
    fileData <- readFile filePath
    --print $ testelem 15 [1,2,3,4,5,6,7,8,9,0]
    --print $ testnub [21,2,2,2,3,3,4,2,5,6,7,8]
    --print $ tisAsc [2,2,2,3,3,4,5,6,7,8]
    let li = map createList $ lines fileData
        sumAsc = countTrue $ map tisAsc li
        sumDesc = countTrue $ map tisDesc li
        sumPermutes = countTrue $ map checkPermutes li
    print $ sumTwo sumAsc sumDesc -- part 1
    print $ sumPermutes


