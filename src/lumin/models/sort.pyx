
cpdef stupidfunc(str name1, str name2, str input_text):
    cdef int score1 = longestCommonSubstr(name1, input_text)
    cdef int score2 = longestCommonSubstr(name2, input_text)

    return score2 - score1

cpdef longestCommonSubstr(str s1, str s2):
    cdef int m = len(s1)
    cdef int n = len(s2)

    # cdef int[n+1] prev

    # Create a 1D array to store the previous row's results
    prev = [0] * (n + 1)

    cdef int res = 0
    for i in range(1, m + 1):
        # Create a temporary array to store the current row
        curr = [0] * (n + 1)
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                curr[j] = prev[j - 1] + 1
                res = max(res, curr[j])
            else:
                curr[j] = 0

        # Move the current row's data to the previous row
        prev = curr

    return res

