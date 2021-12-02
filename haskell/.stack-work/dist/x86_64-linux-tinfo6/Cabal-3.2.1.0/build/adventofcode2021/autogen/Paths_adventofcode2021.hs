{-# LANGUAGE CPP #-}
{-# LANGUAGE NoRebindableSyntax #-}
{-# OPTIONS_GHC -fno-warn-missing-import-lists #-}
module Paths_adventofcode2021 (
    version,
    getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir,
    getDataFileName, getSysconfDir
  ) where

import qualified Control.Exception as Exception
import Data.Version (Version(..))
import System.Environment (getEnv)
import Prelude

#if defined(VERSION_base)

#if MIN_VERSION_base(4,0,0)
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#else
catchIO :: IO a -> (Exception.Exception -> IO a) -> IO a
#endif

#else
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#endif
catchIO = Exception.catch

version :: Version
version = Version [0,1,0,0] []
bindir, libdir, dynlibdir, datadir, libexecdir, sysconfdir :: FilePath

bindir     = "/home/jthein/repos/adventofcode2021/.stack-work/install/x86_64-linux-tinfo6/038d519167f7d5ca1bc7420252f878b65bb072a083c823eb24a9777e88123a2f/8.10.7/bin"
libdir     = "/home/jthein/repos/adventofcode2021/.stack-work/install/x86_64-linux-tinfo6/038d519167f7d5ca1bc7420252f878b65bb072a083c823eb24a9777e88123a2f/8.10.7/lib/x86_64-linux-ghc-8.10.7/adventofcode2021-0.1.0.0-ET9XzG7hDlzLLrfUwh5GBh-adventofcode2021"
dynlibdir  = "/home/jthein/repos/adventofcode2021/.stack-work/install/x86_64-linux-tinfo6/038d519167f7d5ca1bc7420252f878b65bb072a083c823eb24a9777e88123a2f/8.10.7/lib/x86_64-linux-ghc-8.10.7"
datadir    = "/home/jthein/repos/adventofcode2021/.stack-work/install/x86_64-linux-tinfo6/038d519167f7d5ca1bc7420252f878b65bb072a083c823eb24a9777e88123a2f/8.10.7/share/x86_64-linux-ghc-8.10.7/adventofcode2021-0.1.0.0"
libexecdir = "/home/jthein/repos/adventofcode2021/.stack-work/install/x86_64-linux-tinfo6/038d519167f7d5ca1bc7420252f878b65bb072a083c823eb24a9777e88123a2f/8.10.7/libexec/x86_64-linux-ghc-8.10.7/adventofcode2021-0.1.0.0"
sysconfdir = "/home/jthein/repos/adventofcode2021/.stack-work/install/x86_64-linux-tinfo6/038d519167f7d5ca1bc7420252f878b65bb072a083c823eb24a9777e88123a2f/8.10.7/etc"

getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir, getSysconfDir :: IO FilePath
getBinDir = catchIO (getEnv "adventofcode2021_bindir") (\_ -> return bindir)
getLibDir = catchIO (getEnv "adventofcode2021_libdir") (\_ -> return libdir)
getDynLibDir = catchIO (getEnv "adventofcode2021_dynlibdir") (\_ -> return dynlibdir)
getDataDir = catchIO (getEnv "adventofcode2021_datadir") (\_ -> return datadir)
getLibexecDir = catchIO (getEnv "adventofcode2021_libexecdir") (\_ -> return libexecdir)
getSysconfDir = catchIO (getEnv "adventofcode2021_sysconfdir") (\_ -> return sysconfdir)

getDataFileName :: FilePath -> IO FilePath
getDataFileName name = do
  dir <- getDataDir
  return (dir ++ "/" ++ name)
