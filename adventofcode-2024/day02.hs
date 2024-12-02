import System.Environment
import System.IO
import Data.List

parse :: String -> [[Int]]
parse s = map (map read) $ map words $ lines $ s

windows ls = zip ls (tail ls)

allw f g ls = all g $ map f $ windows ls


safe ls = mon && delta
  where mon = allw (uncurry (-)) (<0) ls || allw (uncurry (-)) (>0) ls
        delta = allw (\(a,b) -> abs (a-b)) (<=3) ls

part1 ls = length $ filter safe ls

te [] = []
te xs = init xs

dropping ls = map (\i -> let (l,r) = splitAt i ls in (te l) ++ r) [0..(length ls)]

part2 ls = length $ filter (\l -> any safe $ dropping l) ls
dbg ls = unlines $ map show $ filter (\l -> any safe $ dropping l) ls

main = do
    args <- getArgs
    s <- readFile (args !! 0)

    let ls = parse s
    --putStrLn $ show $ part1 ls
    --putStrLn $ show $ part2 ls
    putStrLn $  dbg ls
