/*!
    A message-only window that dispatch events not targeted to any control
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use winapi::HWND;

use error::Error;

/**
    Object that dispatch events not targeted at any control.

    No automatic resources freeing, `MessageHandle.free` must be called before the struct goes out of scope.
*/
pub struct MessageHandler {
    hwnd: HWND
}

impl MessageHandler {

    /**
        Create a new message handle. 

        * If the window creation was successful, return the new message handler
        * If the system was not capable to create the window, return a `Error::System`
    */
    pub fn new() -> Result<MessageHandler, Error> {
        let hwnd_result = unsafe{ create_message_only_window() };
        match hwnd_result {
            Ok(h) => Ok( MessageHandler{ hwnd: h } ),
            Err(e) => Err(e)
        }
    }

    /**
        Destroy the underlying window and try to free the class. Errors are ignored.

        If multiple UI were created, the class destruction will silently fail and it's ok.
        The class will be freed when the last Ui is freed.
    */
    pub fn free(&self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.hwnd); }
    }
}

unsafe fn create_message_only_window() -> Result<HWND, Error> {
    Err(Error::Unimplemented)
}