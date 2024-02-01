# Intersection of two sorted arrays
def intersection(nums1, nums2):
    i = 0
    j = 0
    result = []
    while i < len(nums1) and j < len(nums2):
        if nums1[i] < nums2[j]:
            i += 1
        elif nums1[i] > nums2[j]:
            j += 1
        else:
            if len(result) == 0  or result[-1] != nums1[i]:
                result.append(nums1[i])
            i += 1
            j += 1
    return result
