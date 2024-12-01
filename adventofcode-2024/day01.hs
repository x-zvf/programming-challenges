import System.Environment
import System.IO
import Data.List

parse s = transpose $ map (map read . words) $ lines $ s

part1 l r = sum $ map abs $ zipWith (-) (sort l) (sort r)

part2 l r = sum $ map (\v -> v * (length $ filter (== v) r)) l

main = do
    args <- getArgs
    s <- readFile (args !! 0)

    let [l,r] = parse s
    putStrLn $ show $ part1 l r
    putStrLn $ show $ part2 l r
