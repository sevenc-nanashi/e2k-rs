from typing import Final, Literal, overload

kanas: Final[list[str]]
ascii_entries: Final[list[str]]
en_phones: Final[list[str]]

class C2k:
    """英単語 -> カタカナの推論を行う。"""

    def __init__(self, max_len: int = 32) -> None:
        """
        新しいインスタンスを生成する。

        Parameters
        ----------
        max_len : int, default 32
            最大の入力長。
        """

        ...

    @overload
    def set_decode_strategy(self, strategy: Literal["greedy"]) -> None: ...
    @overload
    def set_decode_strategy(self, strategy: Literal["top_k"], k: int) -> None: ...
    @overload
    def set_decode_strategy(
        self, strategy: Literal["top_p"], p: float, t: float
    ) -> None: ...
    def set_decode_strategy(self, strategy: str, **kwargs) -> None:
        """
        デコード戦略を設定する。

        Parameters
        ----------
        strategy : str
            デコード戦略。
        **kwargs
            戦略に応じた引数。詳細はメソッドのオーバーロードを参照。
        """
        ...

    def __call__(self, word: str) -> str:
        """
        推論を行う。

        Parameters
        ----------
        word : str
            英単語。

        Returns
        -------
        str
            カタカナ。
        """
        ...

class P2k:
    """音素 -> カタカナの推論を行う。"""

    def __init__(self, max_len: int = 32) -> None:
        """
        新しいインスタンスを生成する。

        Parameters
        ----------
        max_len : int, default 32
            最大の入力長。
        """

        ...

    @overload
    def set_decode_strategy(self, strategy: Literal["greedy"]) -> None: ...
    @overload
    def set_decode_strategy(self, strategy: Literal["top_k"], k: int) -> None: ...
    @overload
    def set_decode_strategy(
        self, strategy: Literal["top_p"], p: float, t: float
    ) -> None: ...
    def set_decode_strategy(self, strategy: str, **kwargs) -> None:
        """
        デコード戦略を設定する。

        Parameters
        ----------
        strategy : str
            デコード戦略。
        **kwargs
            戦略に応じた引数。詳細はメソッドのオーバーロードを参照。
        """
        ...

    def __call__(self, phonemes: list[str]) -> str:
        """
        推論を行う。

        Parameters
        ----------
        phonemes : list[str]
            CMUDictの音素。

        Returns
        -------
        str
            カタカナ。
        """
        ...
