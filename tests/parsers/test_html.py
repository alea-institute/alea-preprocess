import alea_preprocess


EXAMPLE_1 = """
<!DOCTYPE html>
<html>
    <head>
        <title>Page Title</title>
    </head>
    <body>
        <h1>This is a Heading</h1>
        <p>This is a paragraph.</p>
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
        </ul>
    </body>
</html>
"""


def test_example_1():
    output = alea_preprocess.parsers.html.conversion.extract_buffer_markdown(
        EXAMPLE_1, output_links=True, output_images=True
    )

    assert "# This is a Heading\n" in output
    assert "\nThis is a paragraph.\n" in output
    assert "- Item 1\n- Item 2\n" in output


def test_example_file1():
    output = alea_preprocess.parsers.html.conversion.extract_buffer_markdown(
        open("resources/file1.html").read(), output_links=True, output_images=True
    )

    assert "# Our blog\n" in output
    assert (
        "Don't be shy. We'd love to hear from you. [Contact us](/forms/contact)"
        in output
    )


def test_example_treasury_markdown():
    output = alea_preprocess.parsers.html.conversion.extract_buffer_markdown(
        open("resources/treasury.html").read(), True, True
    )

    with open("/tmp/test-treasury.txt", "wt") as output_file:
        output_file.write(output)

    assert "#block-content-homepage-hero" not in output
    assert (
        "U.S. Department of the Treasury Announces Maine Will Join IRS Direct File for Filing Season 2025"
        in output
    )


def test_example_treasury_text():
    output = alea_preprocess.parsers.html.conversion.extract_buffer_text(
        open("resources/treasury.html").read()
    )

    assert (
        "U.S. Department of the Treasury Announces Maine Will Join IRS Direct File for Filing Season 2025"
        in output
    )


def test_example_dol_markdown():
    output = alea_preprocess.parsers.html.conversion.extract_buffer_markdown(
        open("resources/entrepre.htm").read(), output_links=True, output_images=True
    )

    assert "Microenterprise organizations include capital" in output


def test_example_dol_text():
    output = alea_preprocess.parsers.html.conversion.extract_buffer_text(
        open("resources/entrepre.htm").read()
    )

    with open("/tmp/test.txt", "wt", encoding="utf-8") as output_file:
        output_file.write(output)

    assert "Microenterprise organizations include capital" in output
