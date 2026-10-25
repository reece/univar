"""Hello world: build a univrs model in Python, serialize it in Rust, print the returned str."""

import univrs


def main() -> None:
    loc = univrs.SequenceLocation(sequence_id="NC_000001.11", start=100, end=101)
    allele = univrs.Allele(location=loc, state="T")
    json: str = allele.to_json()  # serde_json, in Rust
    print(json)


if __name__ == "__main__":
    main()
