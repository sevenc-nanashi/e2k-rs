import e2k_rs


def test_c2k():
    c2k = e2k_rs.C2k()

    word = "constants"
    assert c2k(word) == "コンスタンツ"


def test_p2k():
    p2k = e2k_rs.P2k()
    pronunciation = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"]
    assert p2k(pronunciation) == "コンスタンツ"
