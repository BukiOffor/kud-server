'use client';

import { useState, useEffect } from 'react';
import { eventsApi } from '@/lib/api';
import { Event } from '@/lib/types';
import { Calendar, MapPin, Clock, UserCheck, Plus, Trash2 } from 'lucide-react';
import Modal from '@/components/ui/Modal';
import CreateEventForm from './CreateEventForm';
import CheckInForm from './CheckInForm';
import EventReportModal from './EventReportModal';

const EventsPage = () => {
  const [events, setEvents] = useState<Event[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isCheckInModalOpen, setIsCheckInModalOpen] = useState(false);
  const [isReportModalOpen, setIsReportModalOpen] = useState(false);
  const [selectedEventId, setSelectedEventId] = useState<string | null>(null);
  const [selectedEventTitle, setSelectedEventTitle] = useState<string>('');
  const [user, setUser] = useState<any>(null);

  useEffect(() => {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      setUser(JSON.parse(storedUser));
    }
    fetchEvents();
  }, []);

  const isAdminOrTech = user?.role === 'Admin' || user?.role === 'Technical';

  const fetchEvents = async () => {
    try {
      setLoading(true);
      const response = await eventsApi.getAll();
      setEvents(response.data);
      setError(null);
    } catch (err) {
      console.error('Failed to fetch events:', err);
      setError('Failed to load events. Please make sure the backend is running.');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this event?')) return;
    try {
      await eventsApi.delete(id);
      setEvents(events.filter(e => e.id !== id));
    } catch (err) {
      alert('Failed to delete event');
    }
  };

  const handleCheckInClick = (eventId: string) => {
    setSelectedEventId(eventId);
    setIsCheckInModalOpen(true);
  };

  const handleViewReport = (eventId: string, title: string) => {
    setSelectedEventId(eventId);
    setSelectedEventTitle(title);
    setIsReportModalOpen(true);
  };

  const isEventConcluded = (dateStr: string, timeStr: string) => {
    try {
      const eventDateTime = new Date(`${dateStr}T${timeStr}`);
      return eventDateTime < new Date();
    } catch {
      return false;
    }
  };

  const isEventActive = (dateStr: string, timeStr: string, graceMinutes: number) => {
    try {
      const start = new Date(`${dateStr}T${timeStr}`);
      const end = new Date(start.getTime() + (graceMinutes || 30) * 60000);
      const now = new Date();
      return now >= start && now <= end;
    } catch {
      return false;
    }
  };

  const handleSelfCheckIn = async (event: Event) => {
    if (!user) {
      alert('Please log in to check in.');
      return;
    }

    setLoading(true);
    try {
      let location: any = undefined;
      
      try {
        const pos: any = await new Promise((resolve, reject) => {
          navigator.geolocation.getCurrentPosition(resolve, reject, { 
            timeout: 10000,
            enableHighAccuracy: true
          });
        });
        location = {
          lat: pos.coords.latitude,
          lng: pos.coords.longitude
        };
      } catch (err) {
        console.warn('Geolocation failed:', err);
      }

      await eventsApi.checkIn({
        event_id: event.id,
        user_id: user.id || user.user_id,
        attendance_type: 'Standard',
        location
      });
      
      alert('Successfully checked in!');
      fetchEvents();
    } catch (err: any) {
      alert(err.response?.data?.message || 'Failed to check in. Please ensure you are at the correct location.');
    } finally {
      setLoading(false);
    }
  };


  return (
    <div>
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Events</h1>
        {isAdminOrTech && (
          <button 
            onClick={() => setIsCreateModalOpen(true)}
            className="flex items-center rounded-lg bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
          >
            <Plus className="me-2 h-5 w-5" />
            Create Event
          </button>
        )}
      </div>

      {error && (
        <div className="mb-4 rounded-lg bg-red-100 p-4 text-red-700 dark:bg-red-900/30 dark:text-red-400">
          {error}
        </div>
      )}

      {loading ? (
        <p className="text-gray-500">Loading events...</p>
      ) : (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
          {events.map((event) => (
            <div key={event.id} className="flex flex-col rounded-lg border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
              <div className="mb-4 flex items-center justify-between">
                <h3 className="text-lg font-bold text-gray-800 dark:text-white">{event.title}</h3>
                <span className="rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-300">
                  {event.attendance_type}
                </span>
              </div>
              
              <p className="mb-6 flex-grow text-gray-600 dark:text-gray-400">{event.description}</p>
              
              <div className="space-y-2 text-sm text-gray-500 dark:text-gray-400">
                <div className="flex items-center">
                  <Calendar className="me-2 h-4 w-4" />
                  {event.date}
                </div>
                <div className="flex items-center">
                  <Clock className="me-2 h-4 w-4" />
                  {event.time} ({event.grace_period_in_minutes}m grace)
                </div>
                <div className="flex items-center">
                  <MapPin className="me-2 h-4 w-4" />
                  {event.location}
                </div>
              </div>

              <div className="mt-6 flex flex-col gap-2">
                {isEventActive(event.date, event.time, event.grace_period_in_minutes) && (
                  <button 
                    onClick={() => handleSelfCheckIn(event)}
                    className="flex w-full items-center justify-center rounded-lg bg-green-600 py-2 text-sm font-semibold text-white hover:bg-green-700"
                  >
                    <UserCheck className="me-2 h-4 w-4" />
                    Check In Now
                  </button>
                )}

                {isAdminOrTech && (
                  <>
                    <div className="flex gap-2">
                      {!isEventActive(event.date, event.time, event.grace_period_in_minutes) && (
                        <button 
                          onClick={() => handleSelfCheckIn(event)}
                          className="flex flex-1 items-center justify-center rounded-lg bg-blue-600 py-2 text-sm font-medium text-white hover:bg-blue-700"
                        >
                          <UserCheck className="me-2 h-4 w-4" />
                          Self Check-in
                        </button>
                      )}
                      <button 
                        onClick={() => handleCheckInClick(event.id)}
                        className="flex flex-1 items-center justify-center rounded-lg border border-gray-200 bg-white py-2 text-sm font-medium text-gray-900 hover:bg-gray-100 hover:text-blue-700 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
                      >
                        Check-in Others
                      </button>
                      <button 
                        onClick={() => handleDelete(event.id)}
                        className="rounded-lg border border-red-200 bg-white p-2 text-red-600 hover:bg-red-50 dark:border-red-900 dark:bg-gray-800 dark:hover:bg-red-900/20"
                      >
                        <Trash2 className="h-4 w-4" />
                      </button>
                    </div>
                    {isEventConcluded(event.date, event.time) && (
                      <button 
                        onClick={() => handleViewReport(event.id, event.title)}
                        className="flex w-full items-center justify-center rounded-lg bg-gray-50 py-2 text-sm font-semibold text-blue-600 hover:bg-blue-50 dark:bg-gray-700/50 dark:text-blue-400 dark:hover:bg-gray-700"
                      >
                        View Attendance Report
                      </button>
                    )}
                  </>
                )}
              </div>
            </div>
          ))}
          {events.length === 0 && !loading && !error && (
            <div className="col-span-full rounded-lg border-2 border-dashed border-gray-200 p-12 text-center dark:border-gray-700">
              <p className="text-gray-500">No events found. Create your first event!</p>
            </div>
          )}
        </div>
      )}

      {/* Modals */}
      <Modal 
        isOpen={isCreateModalOpen} 
        onClose={() => setIsCreateModalOpen(false)} 
        title="Create New Event"
      >
        <CreateEventForm 
          onSuccess={() => {
            setIsCreateModalOpen(false);
            fetchEvents();
          }} 
          onCancel={() => setIsCreateModalOpen(false)} 
        />
      </Modal>

      <Modal 
        isOpen={isCheckInModalOpen} 
        onClose={() => setIsCheckInModalOpen(false)} 
        title="User Check-in"
      >
        {selectedEventId && (
          <CheckInForm 
            eventId={selectedEventId}
            onSuccess={() => {
              setIsCheckInModalOpen(false);
              alert('User checked in successfully!');
            }} 
            onCancel={() => setIsCheckInModalOpen(false)} 
          />
        )}
      </Modal>

      {selectedEventId && (
        <EventReportModal
          isOpen={isReportModalOpen}
          onClose={() => {
            setIsReportModalOpen(false);
            setSelectedEventId(null);
          }}
          eventId={selectedEventId}
          eventTitle={selectedEventTitle}
        />
      )}
    </div>
  );
};

export default EventsPage;
