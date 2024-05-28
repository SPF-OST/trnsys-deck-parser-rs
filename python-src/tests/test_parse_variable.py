from trnsys_deck_parser_rs import Parser, Variable


class TestParseVariable:
    def test_parse_variable(self) -> None:
        parser = Parser()
        actual_variable = parser.parse_variable("VIceSscaled")

        expected_variable = Variable("VIceSscaled")

        assert actual_variable == expected_variable
