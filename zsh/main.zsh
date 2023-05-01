if test -e go.mod; then
    echo "\033[38;2;62;207;168mReviewing modules...\033[0m" && go mod tidy
else
    echo "\033[38;2;212;42;51mError: go.mod file not found\033[0m" >&2
    return 1
fi
