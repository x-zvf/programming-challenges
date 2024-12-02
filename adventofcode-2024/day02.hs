import System.Environment
import System.IO
import Data.List

parse s = map (map read) $ map words $ lines $ s

safe ls = mon && delta
  where 
    windows = zip ls (tail ls)
    allw f g = all g $ map f $ windows
    mon = allw (uncurry (-)) (<0) || allw (uncurry (-)) (>0)
    delta = allw (\(a,b) -> abs (a-b)) (<=3) 


dropping ls =
  map (\i -> let (l,r) = splitAt i ls in (inits l) ++ r) idxs
  where
    idxs = [0..(length ls)]
    inits [] = []
    inits xs = init xs


part1 = length . filter safe
part2 = length . filter (\l -> any safe $ dropping l) 

main = do
    args <- getArgs
    s <- readFile (args !! 0)
    let ls = parse s
    putStrLn $ show $ part1 ls
    putStrLn $ show $ part2 ls
