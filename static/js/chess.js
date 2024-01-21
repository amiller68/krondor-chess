// chess.js
document.addEventListener('DOMContentLoaded', () => {
    let selectedPiece = null;

    document.querySelectorAll('.chess-square').forEach(square => {
        square.addEventListener('click', function() {
            if (this.classList.contains('chess-piece') && !selectedPiece) {
                // Select the piece
                selectedPiece = this;
                this.classList.add('selected');
            } else if (selectedPiece) {
                // Move the piece to the new square
                movePiece(selectedPiece, this);
                selectedPiece.classList.remove('selected');
                selectedPiece = null;
            }
        });
    });

    function movePiece(fromSquare, toSquare) {
        const piece = fromSquare.innerHTML;
        fromSquare.innerHTML = ''; // Remove the piece from the current square
        toSquare.innerHTML = piece; // Place the piece on the new square

        // Send the move to the server for validation and state update
        const fromPosition = fromSquare.getAttribute('data-position');
        const toPosition = toSquare.getAttribute('data-position');
        sendMoveToServer(fromPosition, toPosition);
    }

    function sendMoveToServer(from, to) {
        // This function should be implemented to send the move to the server.
        // For demonstration purposes, this is a mock function.
        console.log(`Move sent to server: ${from} to ${to}`);
        // Implement AJAX request or HTMX trigger here
    }
});

