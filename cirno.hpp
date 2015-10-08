#ifndef CIRNO_HPP
#define CIRNO_HPP
#include <string>
#include <iostream>
#include <../algebraic_data_type/algebraic_data_type.hpp>
using namespace algebraic_data_type;
DECLARE_ADT( expr, ((Print, recursive_indicator), (Seq, recursive_indicator, recursive_indicator), (Unit), (True), (False), (If, recursive_indicator, recursive_indicator, recursive_indicator), (String, std::string)), X )

bool value_to_bool( const expr & e )
{ return e.match( with( True( uim ), []( ){ return true; } ), with( False( uim ), []( ){ return false; } ) ); }

std::string show( const expr & e )
{
    return e.match(
                with( Unit( uim ), []( ) { return "tt"; } ),
                with( True( uim ), []( ) { return "true"; } ),
                with( False( uim ), []( ) { return "false"; } ),
                with( String( arg ), []( const std::string & str ) { return str; } ) );
}

expr execute( const expr & e )
{
    return
        e.match(
            with( Print( arg ), []( const expr & exp ) { std::cout << show( execute( exp ) ); return Unit( ); } ),
            with( Seq( arg, arg ), []( const expr & l, const expr & r ) { execute( l ); return execute( r ); } ),
            with( Unit( uim ), []( ){ return Unit( ); } ),
            with( True( uim ), []( ){ return True( ); } ),
            with( False( uim ), []( ){ return False( ); } ),
            with(
                If( arg, arg, arg ),
                []( const expr & i, const expr & t, const expr & e )
                { return value_to_bool( execute( i ) ) ? execute( t ) : execute( e ); } ),
            with( String( arg ), []( const std::string & str ) { return String( str ); } ));
}
#endif // CIRNO_HPP

