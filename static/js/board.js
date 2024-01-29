// WOOO globabl state
let selectedPiece = null;
let fromSquare = null;
let toSquare = null;

// (Overly) Simple function to check if a square has a piece
function squareHasPiece(square) {
    return square.innerHTML !== '';
}

// Logic for moving a piece
function movePiece(fromSquare, toSquare) {
    // Get the identifying class name (e.g. `chess-piece-P` or `chess-piece-p`) of the piece
    let fromPieceClass = fromSquare.classList[1];
    let fromPiece = fromPieceClass.split('-')[2];
    
    // Get the relevant squares
    let fromPosition = fromSquare.getAttribute('id');
    let toPosition = toSquare.getAttribute('id');
    let toRank = toPosition[1];

    // Determine the uci formatted move
    let promotionHtml = null;
    let promotionClass = null;
    uciMove = `${fromPosition}${toPosition}`;
    // Check if a pawn is being promoted 
    if ((fromPiece === 'P' && toRank === '8') || (fromPiece === 'p' && toRank === '1')) {
        // TODO: allow user to select piece to promote to piece of their choice
        uciMove += 'q'; // Promote to queen 
        if (fromPiece === 'P') {
            promotionHtml = '♕';
            promotionClass = 'chess-piece-Q';
        } else {
            promotionHtml = '♛';
            promotionClass = 'chess-piece-q';
        }
    }

    // Update the board
    let toPieceHtml = promotionHtml ?? fromSquare.innerHTML;
    let toPieceClass = promotionClass ?? fromPieceClass;
    toSquare.innerHTML = toPieceHtml; // Add the piece to the new square
    if (toSquare.classList.length === 1) {
        toSquare.classList.add(toPieceClass);
    } else {
        toSquare.classList.replace(toSquare.classList[1], toPieceClass); // Update the class of the new square
    }
    fromSquare.innerHTML = ''; // Remove the piece from the current square
    sendMove(uciMove); 
}

function sendMove(uciMove) {
    console.log(uciMove);
    // Write our move to the hidden input field
    document.getElementById('uciMoveInput').value = uciMove;
    // Make the button visible
    document.getElementById('moveForm').style.display = 'block';
}

initBoard = function() {
    selectedPiece = null;
    fromSquare = null;
    toSquare = null;

    // Remove all event listeners
    document.querySelectorAll('[class*="chess-square-"]').forEach(square => {
        square.replaceWith(square.cloneNode(true));
    });

    document.getElementById('moveForm').style.display = 'none';
    document.getElementById('uciMoveInput').value = '';

    // Assuming 'chessboard' is the ID of the parent element
    const chessboard = document.getElementById('chessboard');
    chessboard.addEventListener('click', function(event) {
        // Check if the clicked element is a chess square
        const clickedSquare = event.target.closest('[class*="chess-square-"]');
        if (!clickedSquare) return; // Not a chess square, ignore the click
        if (!selectedPiece && squareHasPiece(clickedSquare)) {
            // Select the piece
            selectedPiece = clickedSquare;
            fromSquare = clickedSquare;
            clickedSquare.classList.add('selected');
        } else if (selectedPiece) {
            if (clickedSquare === selectedPiece) {
                // Deselect the piece
                clickedSquare.classList.remove('selected');
                selectedPiece = null;
                return;

            }

            toSquare = clickedSquare;
            // Move the piece to the new square
            movePiece(selectedPiece, clickedSquare);
            selectedPiece.classList.remove('selected');
            selectedPiece = null;
        }
    });

    document.body.addEventListener('htmx:responseError', function(event) {
        // Check if the event is for the element you're interested in
        if (event.target.id === 'submitMove') {
            // Swap the pieces back
            let fromSqaureHtml = fromSquare.innerHTML;
            fromSquare.innerHTML = toSquare.innerHTML;
            toSquare.innerHTML = fromSqaureHtml;
        }
    });
}

document.addEventListener('DOMContentLoaded', function() {
    initBoard();
});

document.body.addEventListener('htmx:afterSwap', function(event) {
    selectedPiece = null;
    fromSquare = null;
    toSquare = null;
})